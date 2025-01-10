use std::collections::{BinaryHeap, HashMap, HashSet};

use anyhow::Result;
use geojson::FeatureCollection;
use graph::RoadID;
use utils::PriorityQueueItem;

use crate::{InfraType, LevelOfService, MapModel};

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
            if let Some(infra_type) = self.infra_types[idx] {
                // TODO If a piece of assigned infrastructure is inappropriate and the level of
                // service is poor, consider it part of the network or not?
                if infra_type != InfraType::MixedTraffic {
                    network.insert(id);
                    queue.push(id);
                    continue;
                }
            }

            if self.los[idx] != LevelOfService::High {
                severances.insert(id);
            }
        }

        // Flood, avoiding severances
        while let Some(r) = queue.pop() {
            // TODO Simplify and just put these in visited upfront?
            if visited.contains(&r) || severances.contains(&r) {
                continue;
            }
            visited.insert(r);

            let road = &self.graph.roads[r.0];
            if !network.contains(&r) {
                reachable.insert(r);
            }
            for i in [road.src_i, road.dst_i] {
                queue.extend(self.graph.intersections[i.0].roads.clone());
            }
        }

        Reachability {
            network,
            severances,
            reachable,
        }
    }

    pub fn render_reachable_network(&self) -> Result<String> {
        let mut features = Vec::new();
        let out = self.get_reachable_network();

        for (kind, list) in [
            ("network", out.network),
            ("severance", out.severances),
            ("reachable", out.reachable),
        ] {
            for r in list {
                let mut f = self
                    .graph
                    .mercator
                    .to_wgs84_gj(&self.graph.roads[r.0].linestring);
                f.set_property("kind", kind);
                features.push(f);
            }
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
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
            let r = item.value;
            if visited.contains(&r) {
                continue;
            }
            visited.insert(r);

            if let Some(infra_type) = self.infra_types[r.0] {
                if infra_type != InfraType::MixedTraffic {
                    // We don't even need the path in order; just draw all of the roads part of the
                    // path
                    features.push(
                        self.graph
                            .mercator
                            .to_wgs84_gj(&self.graph.roads[r.0].linestring),
                    );
                    let mut current = r;
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
            }

            if self.los[r.0] != LevelOfService::High {
                continue;
            }

            let road = &self.graph.roads[r.0];
            for i in [road.src_i, road.dst_i] {
                for r2 in &self.graph.intersections[i.0].roads {
                    if start_roads.contains(r2) {
                        continue;
                    }
                    if !backrefs.contains_key(&r2) {
                        backrefs.insert(*r2, r);
                        queue.push(PriorityQueueItem::new(
                            item.cost + meters(road.length_meters),
                            *r2,
                        ));
                    }
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

            for i in [road.src_i, road.dst_i] {
                queue.extend(self.graph.intersections[i.0].roads.clone());
            }
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
    }
}

// to cm
fn meters(x: f64) -> usize {
    (x * 100.0).round() as usize
}
