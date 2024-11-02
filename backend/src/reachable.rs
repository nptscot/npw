use std::collections::HashSet;

use anyhow::Result;
use geojson::{Feature, FeatureCollection, Geometry};
use graph::RoadID;

use crate::{level_of_service, InfraType, MapModel};

impl MapModel {
    pub fn get_reachable_network(&self) -> Result<String> {
        let mut starts: HashSet<RoadID> = HashSet::new();
        let mut severances: HashSet<RoadID> = HashSet::new();
        let mut visited: HashSet<RoadID> = HashSet::new();
        let mut queue: Vec<RoadID> = Vec::new();
        let mut features = Vec::new();

        let infra_types = self.get_infra_types();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let id = RoadID(idx);
            if let Some(infra_type) = infra_types.get(&id) {
                // TODO If a piece of assigned infrastructure is inappropriate and the level of
                // service is poor, consider it part of the network or not?
                if *infra_type != InfraType::MixedTraffic {
                    starts.insert(id);
                    queue.push(id);
                    let mut f = Feature::from(Geometry::from(
                        &self.graph.mercator.to_wgs84(&road.linestring),
                    ));
                    f.set_property("kind", "network");
                    features.push(f);
                    continue;
                }
            }

            let los = level_of_service::level_of_service(
                InfraType::MixedTraffic,
                self.traffic_volumes[idx],
                level_of_service::get_speed_mph(road),
            );
            if los != level_of_service::LevelOfService::High {
                severances.insert(id);
                let mut f = Feature::from(Geometry::from(
                    &self.graph.mercator.to_wgs84(&road.linestring),
                ));
                f.set_property("kind", "severance");
                features.push(f);
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
            if !starts.contains(&r) {
                let mut f = Feature::from(Geometry::from(
                    &self.graph.mercator.to_wgs84(&road.linestring),
                ));
                f.set_property("kind", "reachable");
                features.push(f);
            }
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
