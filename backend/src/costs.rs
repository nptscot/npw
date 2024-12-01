use std::time::Duration;

use geo::{Euclidean, Length};
use graph::{Road, RoadID, Timer};

use crate::{InfraType, LevelOfService, MapModel};

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
        InfraType::CycleLane => {
            if road
                .osm_tags
                .is_any("highway", vec!["primary", "primary_link"])
            {
                40
            } else if road
                .osm_tags
                .is_any("highway", vec!["secondary", "secondary_link"])
            {
                50
            } else if road
                .osm_tags
                .is_any("highway", vec!["tertiary", "tertiary_link"])
            {
                60
            } else {
                // TODO Assume worst case
                40
            }
        }
        InfraType::MixedTraffic => {
            if road
                .osm_tags
                .is_any("highway", vec!["primary", "primary_link"])
            {
                20
            } else if road
                .osm_tags
                .is_any("highway", vec!["secondary", "secondary_link"])
            {
                30
            } else if road
                .osm_tags
                .is_any("highway", vec!["tertiary", "tertiary_link"])
            {
                40
            } else if road.osm_tags.is("highway", "unclassified") {
                70
            } else if road
                .osm_tags
                .is_any("highway", vec!["residential", "service"])
            {
                // See special case: https://github.com/nptscot/npw/issues/29#issuecomment-2508180511
                60
            } else if road.osm_tags.is("highway", "cycleway") {
                // TODO What does MixedTraffic mean in this case?
                85
            } else {
                // TODO Assume worst case
                40
            }
        }
        // TODO Some kind of infrastructure, but unspecified. Make up a value for now.
        InfraType::Unknown => 50,
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
