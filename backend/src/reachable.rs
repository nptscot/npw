use std::collections::HashSet;

use anyhow::Result;
use geojson::{Feature, FeatureCollection, Geometry};
use graph::RoadID;

use crate::{InfraType, LevelOfService, MapModel};

pub struct Reachability {
    pub network: HashSet<RoadID>,
    pub severances: HashSet<RoadID>,
    pub reachable: HashSet<RoadID>,
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

            if self.level_of_service(id) != LevelOfService::High {
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
                let mut f = Feature::from(Geometry::from(
                    &self
                        .graph
                        .mercator
                        .to_wgs84(&self.graph.roads[r.0].linestring),
                ));
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
}
