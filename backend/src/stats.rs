use anyhow::Result;
use enum_map::EnumMap;
use graph::Timer;

use crate::{precalculated_flow::FlowStats, InfraType, LevelOfService, MapModel};

impl MapModel {
    /// After any edit, calculate summary stats. Returns JSON.
    pub fn recalculate_stats(&mut self, timer: &mut Timer) -> Result<String> {
        let mut out = serde_json::Map::new();

        self.recalculate_router(timer);

        timer.step("calculate reachable network");
        let roads = self.get_reachable_network();
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

        timer.step("calculate OD routes and stats");

        let mut count_by_infra: EnumMap<InfraType, usize> = EnumMap::default();
        let mut count_by_los: EnumMap<LevelOfService, usize> = EnumMap::default();
        let mut count_off_network = 0;
        let mut total_count = 0;

        let od = self.od_counts()?;
        for (r, count) in od.counts {
            total_count += count;
            if let Some(infra_type) = self.infra_types[r.0] {
                count_by_infra[infra_type] += count;
            } else {
                count_off_network += count;
            }
            count_by_los[self.los[r.0]] += count;
        }

        let mut od_percents_infra_type = serde_json::Map::new();
        od_percents_infra_type.insert(
            "Off network".to_string(),
            percent(count_off_network, total_count).into(),
        );
        for (infra_type, count) in count_by_infra {
            od_percents_infra_type.insert(
                format!("{infra_type:?}"),
                percent(count, total_count).into(),
            );
        }

        let mut od_percents_los = serde_json::Map::new();
        for (los, count) in count_by_los {
            od_percents_los.insert(format!("{los:?}"), percent(count, total_count).into());
        }

        out.insert(
            "od_percents_infra_type".to_string(),
            serde_json::Value::Object(od_percents_infra_type),
        );
        out.insert(
            "od_percents_los".to_string(),
            serde_json::Value::Object(od_percents_los),
        );
        out.insert(
            "average_weighted_directness".to_string(),
            od.average_weighted_directness.into(),
        );
        out.insert(
            "worst_directness_routes".to_string(),
            serde_json::to_value(&od.worst_directness_routes)?,
        );

        let flow_stats = FlowStats::new(&self.precalculated_flows);
        let mut covered_quintile_sums = [0; 5];
        for (idx, flow) in self.precalculated_flows.iter().enumerate() {
            // TODO Check definition here -- should this look at LoS, so small high-flow roads are
            // fine?
            let covered = self.infra_types[idx].is_some();
            if covered {
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
}

fn percent(x: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (x as f64) / (total as f64)
    }
}
