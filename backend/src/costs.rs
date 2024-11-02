use std::time::Duration;

use geo::{Euclidean, Length};
use graph::{Road, RoadID};

use crate::{level_of_service, InfraType, MapModel};

impl MapModel {
    /// After some kind of edit, recalculate edge costs. Overwrites the only router.
    pub fn recalculate_router(&mut self) {
        info!("Recalculating router with new edge costs");

        let infra_types = self.get_infra_types();
        for (idx, road) in self.graph.roads.iter_mut().enumerate() {
            let cost = edge_cost(
                road,
                self.traffic_volumes[idx],
                infra_types
                    .get(&RoadID(idx))
                    .cloned()
                    .unwrap_or(InfraType::MixedTraffic),
            );
            road.cost = vec![cost];
        }

        let profile = self.graph.profile_names["bicycle"];
        self.graph.routers[profile.0].update_costs(&self.graph.roads, profile);
    }
}

fn edge_cost(road: &Road, traffic: usize, infra_type: InfraType) -> Duration {
    let speed = level_of_service::get_speed_mph(road);
    let los = level_of_service::level_of_service(infra_type, traffic, speed);

    // TODO Just making these up for now!
    let penalty = match los {
        level_of_service::LevelOfService::High => 1.0,
        level_of_service::LevelOfService::Medium => 1.5,
        level_of_service::LevelOfService::Low => 3.0,
        level_of_service::LevelOfService::ShouldNotBeUsed => 5.0,
    };

    // TODO Ignore cyclist speed for now. Later, do include it -- slower on SharedFootway or uphill
    Duration::from_secs_f64(penalty * road.linestring.length::<Euclidean>())
}
