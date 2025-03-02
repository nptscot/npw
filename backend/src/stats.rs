use anyhow::Result;
use graph::{RoadID, Timer};
use serde::{Deserialize, Serialize};

use crate::{utils::Quintiles, LevelOfService, MapModel, Tier};

/// A summary of metrics. All percents are 0 to 1.
#[derive(Default, Serialize, Deserialize)]
pub struct Stats {
    percent_reachable_schools: f64,
    percent_reachable_gp_hospitals: f64,
    percent_reachable_town_centres: f64,
    percent_reachable_settlements: f64,
    percent_reachable_greenspaces: f64,
    percent_reachable_imd_population: f64,
    percent_reachable_population: f64,

    covered_flow_quintile_sums: Vec<usize>,
    total_flow_quintile_sums: Vec<usize>,

    total_main_road_length: f64,
    covered_main_road_length: f64,

    total_network_length: f64,
    total_high_los_length: f64,
    total_low_gradient_length: f64,
    total_undeliverable_length: f64,

    density_network_in_settlements: Option<f64>,
}

impl MapModel {
    /// After any edit, calculate summary stats. This must be fast, since it's automatically done
    /// after every edit.
    pub fn get_stats(&self, timer: &mut Timer) -> Stats {
        timer.step("get reachable network");
        let roads = self.get_reachable_network();

        timer.step("reachable stats");
        let percent_reachable_schools = percent(
            self.schools.iter().filter(|x| roads.covers(x.road)).count(),
            self.schools.len(),
        );
        let percent_reachable_gp_hospitals = percent(
            self.gp_hospitals
                .iter()
                .filter(|x| roads.covers(x.road))
                .count(),
            self.gp_hospitals.len(),
        );
        let percent_reachable_town_centres = percent(
            self.town_centres
                .iter()
                .filter(|x| roads.covers_any(&x.roads))
                .count(),
            self.town_centres.len(),
        );
        // TODO Weight by population?
        let percent_reachable_settlements = percent(
            self.settlements
                .iter()
                .filter(|x| roads.covers_any(&x.roads))
                .count(),
            self.settlements.len(),
        );
        let percent_reachable_greenspaces = percent(
            self.greenspaces
                .iter()
                .filter(|x| roads.covers_any(&x.roads))
                .count(),
            self.greenspaces.len(),
        );

        // Weighted by population, not just count
        let mut deprived_sum = 0;
        let mut deprived_total = 0;
        let mut population_sum = 0;
        let mut population_total = 0;
        for zone in &self.data_zones {
            // Only the first quintile
            if zone.imd_percentile <= 20 {
                deprived_total += zone.population;
                if roads.covers_any(&zone.roads) {
                    deprived_sum += zone.population;
                }
            }

            population_total += zone.population;
            if roads.covers_any(&zone.roads) {
                population_sum += zone.population;
            }
        }
        let percent_reachable_imd_population = percent(deprived_sum, deprived_total).into();
        let percent_reachable_population = percent(population_sum, population_total).into();

        // TODO Cache this?
        timer.step("covered flows");
        let flow_stats = Quintiles::new(&self.precalculated_flows);
        let mut covered_quintile_sums = [0; 5];
        for (idx, flow) in self.precalculated_flows.iter().enumerate() {
            // Only count coverage where something has been explicitly drawn, no matter what that
            // is
            if self.infra_types[idx].is_some() {
                let quintile = flow_stats.quintile(*flow);
                covered_quintile_sums[quintile - 1] += *flow;
            }
        }

        let mut total_network_length = 0.0;
        let mut total_high_los_length = 0.0;
        let mut total_low_gradient_length = 0.0;
        let mut total_undeliverable_length = 0.0;
        let mut total_main_road_length = 0.0;
        let mut covered_main_road_length = 0.0;
        let mut length_in_settlements = 0.0;
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let part_of_network = self.infra_types[idx].is_some();

            if part_of_network {
                total_network_length += road.length_meters;

                if self.gradients[idx].abs() <= 3.0 {
                    total_low_gradient_length += road.length_meters;
                }

                if self.los[idx] == LevelOfService::High {
                    total_high_los_length += road.length_meters;
                }

                if matches!(self.tiers[idx], Some(Tier::Primary | Tier::Secondary))
                    && self.within_settlement[idx]
                {
                    length_in_settlements += road.length_meters;
                }

                if !self.does_infra_type_fit(RoadID(idx), self.infra_types[idx].unwrap()) {
                    total_undeliverable_length += road.length_meters;
                }
            }

            if self.highways[idx].is_main_road() && self.within_settlement[idx] {
                total_main_road_length += road.length_meters;
                if part_of_network {
                    covered_main_road_length += road.length_meters;
                }
            }
        }

        let density_network_in_settlements = if length_in_settlements > 0.0 {
            Some(self.total_settlement_area_m2 / length_in_settlements)
        } else {
            None
        };

        Stats {
            percent_reachable_schools,
            percent_reachable_gp_hospitals,
            percent_reachable_town_centres,
            percent_reachable_settlements,
            percent_reachable_greenspaces,
            percent_reachable_imd_population,
            percent_reachable_population,

            covered_flow_quintile_sums: covered_quintile_sums.to_vec(),
            total_flow_quintile_sums: flow_stats.total_quintile_sums.to_vec(),

            total_network_length,
            total_high_los_length,
            total_low_gradient_length,
            total_undeliverable_length,

            total_main_road_length,
            covered_main_road_length,

            density_network_in_settlements,
        }
    }

    /// Returns JSON. This is slow and user-triggered.
    pub fn recalculate_od_stats(&mut self, timer: &mut Timer) -> Result<String> {
        self.recalculate_router(timer);

        timer.step("calculate OD routes and stats");
        let fast_sample = true;
        let od = self.od_counts(fast_sample)?;
        let mut out = serde_json::Map::new();
        od.describe(self, &mut out)?;
        Ok(serde_json::to_string(&out)?)
    }
}

pub fn percent(x: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (x as f64) / (total as f64)
    }
}
