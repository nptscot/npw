use anyhow::Result;
use geojson::{Feature, FeatureCollection, Geometry};

use crate::MapModel;

impl MapModel {
    pub fn render_precalculated_flows(&self) -> Result<String> {
        // TODO Could cache
        let stats = Stats::new(&self.precalculated_flows);

        let mut covered_quintile_sums = [0; 5];

        let mut features = Vec::new();
        for (idx, (road, flow)) in self
            .graph
            .roads
            .iter()
            .zip(self.precalculated_flows.iter())
            .enumerate()
        {
            let mut f = Feature::from(Geometry::from(
                &self.graph.mercator.to_wgs84(&road.linestring),
            ));
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
                    "quintile_sums": stats.quintile_sums,
                    "covered_quintile_sums": covered_quintile_sums,
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
        })?)
    }
}

struct Stats {
    // 20% of flows are >= this value
    quintile1: usize,
    // 40% of flows are >= this value
    quintile2: usize,
    quintile3: usize,
    quintile4: usize,
    // quintile5 is the minimum flow
    quintile_sums: [usize; 5],
}

impl Stats {
    fn new(flows: &Vec<usize>) -> Self {
        let mut sorted = flows.clone();
        sorted.sort();
        sorted.reverse();
        let n = sorted.len() / 5;

        let mut quintile_sums = [0; 5];
        for (idx, x) in sorted.iter().enumerate() {
            quintile_sums[idx / n] += *x;
        }

        Self {
            quintile1: sorted[n],
            quintile2: sorted[n * 2],
            quintile3: sorted[n * 3],
            quintile4: sorted[n * 4],
            quintile_sums,
        }
    }

    // Returns [1, 5]
    fn quintile(&self, flow: usize) -> usize {
        if flow >= self.quintile1 {
            1
        } else if flow >= self.quintile2 {
            2
        } else if flow >= self.quintile3 {
            3
        } else if flow >= self.quintile4 {
            4
        } else {
            5
        }
    }
}
