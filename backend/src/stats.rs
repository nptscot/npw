use anyhow::Result;
use graph::Timer;

use crate::{utils::Quintiles, MapModel};

impl MapModel {
    /// After any edit, calculate summary stats. Returns JSON. This must be fast, since it's
    /// automatically done after every edit.
    pub fn recalculate_stats(&mut self, timer: &mut Timer) -> Result<String> {
        let mut out = serde_json::Map::new();

        timer.step("get reachable network");
        let roads = self.get_reachable_network();
        timer.step("reachable stats");
        out.insert(
            "percent_reachable_schools".to_string(),
            percent(
                self.schools.iter().filter(|x| roads.covers(x.road)).count(),
                self.schools.len(),
            )
            .into(),
        );
        out.insert(
            "percent_reachable_gp_hospitals".to_string(),
            percent(
                self.gp_hospitals
                    .iter()
                    .filter(|x| roads.covers(x.road))
                    .count(),
                self.gp_hospitals.len(),
            )
            .into(),
        );
        out.insert(
            "percent_reachable_town_centres".to_string(),
            percent(
                self.town_centres
                    .iter()
                    .filter(|x| roads.covers_any(&x.roads))
                    .count(),
                self.town_centres.len(),
            )
            .into(),
        );
        // TODO Weight by population?
        out.insert(
            "percent_reachable_settlements".to_string(),
            percent(
                self.settlements
                    .iter()
                    .filter(|x| roads.covers_any(&x.roads))
                    .count(),
                self.settlements.len(),
            )
            .into(),
        );
        out.insert(
            "percent_reachable_greenspaces".to_string(),
            percent(
                self.greenspaces
                    .iter()
                    .filter(|x| roads.covers_any(&x.roads))
                    .count(),
                self.greenspaces.len(),
            )
            .into(),
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
        out.insert(
            "percent_reachable_imd_population".to_string(),
            percent(deprived_sum, deprived_total).into(),
        );
        out.insert(
            "percent_reachable_population".to_string(),
            percent(population_sum, population_total).into(),
        );

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
        out.insert(
            "covered_flow_quintile_sums".to_string(),
            covered_quintile_sums.to_vec().into(),
        );
        out.insert(
            "total_flow_quintile_sums".to_string(),
            flow_stats.total_quintile_sums.to_vec().into(),
        );

        Ok(serde_json::to_string(&out)?)
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
