use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::Result;
use geojson::FeatureCollection;
use graph::RoadID;
use utils::PriorityQueueItem;

use crate::{LevelOfService, MapModel};

pub struct Reachability {
    pub network: HashSet<RoadID>,
    pub severances: HashSet<RoadID>,
    pub reachable: HashSet<RoadID>,
}

impl Reachability {
    pub fn covers(&self, r: RoadID) -> bool {
        self.network.contains(&r) || self.reachable.contains(&r)
    }

    pub fn covers_any(&self, roads: &HashSet<RoadID>) -> bool {
        !self.network.is_disjoint(roads) || !self.reachable.is_disjoint(roads)
    }
}

impl MapModel {
    pub fn get_reachable_network(&self) -> Reachability {
        let mut network: HashSet<RoadID> = HashSet::new();
        let mut severances: HashSet<RoadID> = HashSet::new();
        let mut reachable: HashSet<RoadID> = HashSet::new();

        let mut visited: HashSet<RoadID> = HashSet::new();
        let mut queue: Vec<RoadID> = Vec::new();

        for idx in 0..self.graph.roads.len() {
            let id = RoadID(idx);

            // Even if there's a piece of network drawn, if it doesn't achieve high LoS, don't
            // count it as part of the network
            if self.los[idx] != LevelOfService::High {
                severances.insert(id);
                visited.insert(id);
                continue;
            }

            if self.infra_types[idx].is_some() {
                network.insert(id);
                queue.push(id);
            }
        }

        // Flood, avoiding severances
        while let Some(r) = queue.pop() {
            if visited.contains(&r) {
                continue;
            }
            visited.insert(r);

            if !network.contains(&r) {
                reachable.insert(r);
            }

            queue.extend(self.next_reachable_roads(r));
        }

        Reachability {
            network,
            severances,
            reachable,
        }
    }

    /// Show the shortest distance path from any of the start roads to any part of the network.
    pub fn debug_reachable_path(&self, start_roads: HashSet<RoadID>) -> Result<String> {
        let mut visited: HashSet<RoadID> = HashSet::new();
        let mut backrefs: HashMap<RoadID, RoadID> = HashMap::new();
        let mut queue: BinaryHeap<PriorityQueueItem<usize, RoadID>> = BinaryHeap::new();
        for r in &start_roads {
            queue.push(PriorityQueueItem::new(0, *r));
        }

        let mut features = Vec::new();

        while let Some(item) = queue.pop() {
            let r1 = item.value;
            if visited.contains(&r1) {
                continue;
            }
            visited.insert(r1);

            if self.los[r1.0] != LevelOfService::High {
                continue;
            }

            if self.infra_types[r1.0].is_some() {
                // We don't even need the path in order; just draw all of the roads part of the
                // path
                features.push(
                    self.graph
                        .mercator
                        .to_wgs84_gj(&self.graph.roads[r1.0].linestring),
                );
                let mut current = r1;
                while let Some(next) = backrefs.get(&current) {
                    features.push(
                        self.graph
                            .mercator
                            .to_wgs84_gj(&self.graph.roads[next.0].linestring),
                    );
                    current = *next;
                }
                break;
            }

            let road1 = &self.graph.roads[r1.0];
            for r2 in self.next_reachable_roads(r1) {
                if start_roads.contains(&r2) {
                    continue;
                }
                if !backrefs.contains_key(&r2) {
                    backrefs.insert(r2, r1);
                    queue.push(PriorityQueueItem::new(
                        item.cost + meters(road1.length_meters),
                        r2,
                    ));
                }
            }
        }

        if features.is_empty() {
            warn!("debug_reachable_path failed!");
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
    }

    /// Flood from all of the start roads, showing reachable roads only
    pub fn debug_unreachable_path(&self, start_roads: HashSet<RoadID>) -> Result<String> {
        let mut features = Vec::new();

        let mut visited: HashSet<RoadID> = HashSet::new();
        let mut queue: Vec<RoadID> = Vec::new();
        queue.extend(start_roads);

        while let Some(r) = queue.pop() {
            // Do this upfront, because something might snap to a severance
            if self.los[r.0] != LevelOfService::High {
                continue;
            }

            if visited.contains(&r) {
                continue;
            }
            visited.insert(r);

            let road = &self.graph.roads[r.0];
            features.push(self.graph.mercator.to_wgs84_gj(&road.linestring));

            queue.extend(self.next_reachable_roads(r));
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
    }

    /// From a road, find all possible next roads that're reachable. This avoids crossing
    /// perpendicular over anything that isn't high LoS.
    fn next_reachable_roads(&self, r1: RoadID) -> Vec<RoadID> {
        let road1 = &self.graph.roads[r1.0];
        let mut results = Vec::new();
        for i in [road1.src_i, road1.dst_i] {
            let roads_clockwise = &self.graph.intersections[i.0].roads;
            for r2 in roads_clockwise {
                if *r2 == r1 {
                    continue;
                }

                // If both r1 and r2 explicitly have infrastructure, then assume there's a crossing
                if self.infra_types[r1.0].is_some() && self.infra_types[r2.0].is_some() {
                    results.push(*r2);
                    continue;
                }

                // Otherwise, check what roads we need to cross. If any of them aren't high LoS,
                // don't allow this movement.
                if all_crossed_roads(roads_clockwise, r1, *r2)
                    .into_iter()
                    .all(|r| self.los[r.0] == LevelOfService::High)
                {
                    results.push(*r2);
                }
            }
        }
        results
    }
}

// to cm
fn meters(x: f64) -> usize {
    (x * 100.0).round() as usize
}

fn all_crossed_roads(clockwise: &Vec<RoadID>, r1: RoadID, r2: RoadID) -> Vec<RoadID> {
    // TODO Do we care about left vs right turns?

    // No possible crossings for small intersections
    if clockwise.len() < 4 {
        return Vec::new();
    }

    let mut idx1 = clockwise.iter().position(|&x| x == r1).unwrap();
    let mut idx2 = clockwise.iter().position(|&x| x == r2).unwrap();

    // If these are adjacent, then no crossings
    if idx2 < idx1 {
        std::mem::swap(&mut idx1, &mut idx2);
    }
    if idx1 == 0 && idx2 == clockwise.len() - 1 {
        return Vec::new();
    }
    if idx2 == idx1 + 1 {
        return Vec::new();
    }

    // Otherwise, every other road aside from r1 and r2 could be crossed
    let mut crossed = clockwise.clone();
    crossed.retain(|x| *x != r1 && *x != r2);
    crossed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_crossed_roads_4_way() {
        // 4-way
        //
        //   N
        //   |
        // W---E
        //   |
        //   S
        let n = RoadID(0);
        let s = RoadID(1);
        let e = RoadID(2);
        let w = RoadID(3);

        let clockwise = vec![n, e, s, w];
        // Note the expected outputs need to follow clockwise ordering
        assert_eq!(all_crossed_roads(&clockwise, n, s), vec![e, w]);
        assert_eq!(all_crossed_roads(&clockwise, s, n), vec![e, w]);
        assert_eq!(all_crossed_roads(&clockwise, e, w), vec![n, s]);
        assert_eq!(all_crossed_roads(&clockwise, w, e), vec![n, s]);

        assert_eq!(all_crossed_roads(&clockwise, n, w), Vec::new());
        assert_eq!(all_crossed_roads(&clockwise, w, n), Vec::new());
        assert_eq!(all_crossed_roads(&clockwise, w, s), Vec::new());
        assert_eq!(all_crossed_roads(&clockwise, s, w), Vec::new());
    }

    #[test]
    fn test_all_crossed_roads_2_or_3_way() {
        // 3-way
        //
        // W---E
        //   |
        //   S
        let s = RoadID(0);
        let e = RoadID(1);
        let w = RoadID(2);

        let clockwise = vec![e, s, w];
        assert_eq!(all_crossed_roads(&clockwise, w, e), Vec::new());
        assert_eq!(all_crossed_roads(&clockwise, w, s), Vec::new());

        let clockwise = vec![e, w];
        assert_eq!(all_crossed_roads(&clockwise, w, e), Vec::new());
    }
}
