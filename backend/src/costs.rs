use std::time::Duration;

use graph::{Road, Timer};

use crate::{LevelOfService, MapModel};

impl MapModel {
    /// After some kind of edit, recalculate edge costs for bicycle_quiet.
    pub fn recalculate_quiet_router(&mut self, timer: &mut Timer) {
        if self.quiet_router_ok {
            return;
        }
        self.quiet_router_ok = true;

        timer.step("recalculate edge costs for bicycle_quiet");

        let mut costs = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            costs.push(quiet_edge_cost(road, self.los[idx]));
        }
        for (road, cost) in self.graph.roads.iter_mut().zip(costs.into_iter()) {
            road.cost = vec![cost];
        }

        timer.step("recalculate CH");
        let profile = self.graph.profile_names["bicycle_quiet"];
        self.graph.routers[profile.0].update_costs(&self.graph.roads, profile);
    }
}

fn quiet_edge_cost(road: &Road, los: LevelOfService) -> Duration {
    // TODO Ignore cyclist speed for now. Later, do include it -- slower on SharedFootway or uphill
    Duration::from_secs_f64(los.penalty() * road.length_meters)
}
