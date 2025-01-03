use std::time::Duration;

use geo::{Euclidean, Length};
use graph::{Road, RoadID, Timer};

use crate::{Highway, InfraType, LevelOfService, MapModel};

impl MapModel {
    /// After some kind of edit, recalculate edge costs. Overwrites the only router.
    pub fn recalculate_router(&mut self, timer: &mut Timer) {
        timer.step("recalculate edge costs");

        let mut costs = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            costs.push(edge_cost(
                road,
                self.get_infra_type(RoadID(idx)),
                self.los[idx],
            ));
        }
        for (road, cost) in self.graph.roads.iter_mut().zip(costs.into_iter()) {
            road.cost = vec![cost];
        }

        timer.step("recalculate CH");
        let profile = self.graph.profile_names["bicycle"];
        self.graph.routers[profile.0].update_costs(&self.graph.roads, profile);
    }
}

fn edge_cost(road: &Road, infra_type: InfraType, los: LevelOfService) -> Duration {
    // Courtesy CycleStreets
    let _quietness = match infra_type {
        InfraType::SegregatedWide => 100,
        InfraType::OffRoad => 100,
        InfraType::SegregatedNarrow => 80,
        InfraType::SharedFootway => 80,
        InfraType::CycleLane => match Highway::classify(&road.osm_tags).unwrap() {
            Highway::Primary => 40,
            Highway::Secondary => 50,
            Highway::Tertiary => 60,
            // TODO Assume worst case. That's clearly wrong; need to revisit all of these cases.
            _ => 40,
        },
        InfraType::MixedTraffic => match Highway::classify(&road.osm_tags).unwrap() {
            // TODO Guessing for these
            Highway::Motorway | Highway::Trunk => 10,
            Highway::Primary => 20,
            Highway::Secondary => 30,
            Highway::Tertiary => 40,
            Highway::Unclassified => 70,
            // See special case: https://github.com/nptscot/npw/issues/29#issuecomment-2508180511
            Highway::Residential | Highway::Service => 60,
            // TODO Check these assumptions. What does MixedTraffic even mean in this case?
            Highway::Cycleway | Highway::Footway | Highway::Pedestrian | Highway::Path => 85,
        },
    };

    // TODO Just making these up for now!
    let penalty = match los {
        LevelOfService::High => 1.0,
        LevelOfService::Medium => 1.5,
        LevelOfService::Low => 3.0,
        LevelOfService::ShouldNotBeUsed => 5.0,
    };

    // TODO Ignore cyclist speed for now. Later, do include it -- slower on SharedFootway or uphill
    Duration::from_secs_f64(penalty * road.linestring.length::<Euclidean>())
}
