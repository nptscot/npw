use anyhow::Result;
use enum_map::EnumMap;
use graph::Timer;

use crate::{InfraType, LevelOfService, MapModel};

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
                self.schools.iter().filter(|s| roads.covers(s.road)).count(),
                self.schools.len(),
            )
            .into(),
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
