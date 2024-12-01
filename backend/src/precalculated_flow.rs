use anyhow::Result;
use geojson::FeatureCollection;

use crate::{utils::Quintiles, MapModel};

impl MapModel {
    pub fn render_precalculated_flows(&self) -> Result<String> {
        // TODO Could cache
        let stats = Quintiles::new(&self.precalculated_flows);
        let mut covered_quintile_sums = [0; 5];

        let mut features = Vec::new();
        for (idx, (road, flow)) in self
            .graph
            .roads
            .iter()
            .zip(self.precalculated_flows.iter())
            .enumerate()
        {
            let mut f = self.graph.mercator.to_wgs84_gj(&road.linestring);
            f.set_property("flow", *flow);
            // TODO Check definition here -- should this look at LoS, so small high-flow roads are
            // fine?
            let covered = self.infra_types[idx].is_some();
            f.set_property("covered", covered);

            let quintile = stats.quintile(*flow);
            f.set_property("quintile", quintile);

            if covered {
                covered_quintile_sums[quintile - 1] += *flow;
            }

            features.push(f);
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: Some(
                serde_json::json!({
                    "total_quintile_sums": stats.total_quintile_sums,
                    "covered_quintile_sums": covered_quintile_sums,
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
        })?)
    }
}
