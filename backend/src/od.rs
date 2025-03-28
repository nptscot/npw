use std::collections::HashMap;

use anyhow::Result;
use enum_map::EnumMap;
use geo::{BoundingRect, Contains, Coord, Intersects, MultiPolygon};
use geojson::{FeatureCollection, Value};
use graph::{PathStep, RoadID};
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use utils::Mercator;

use crate::{stats::percent, uptake, InfraType, LevelOfService, MapModel, Tier};

pub struct CountsOD {
    pub counts: HashMap<RoadID, usize>,
    pub succeeded: usize,
    pub failed: usize,
    pub average_weighted_directness: f64,

    pub worst_directness_routes: Vec<(Coord, Coord)>,
}

impl CountsOD {
    /// Populate `out` with `od_percents_los`, `od_percents_infra_type`, `od_percents_tier`,
    /// `average_weighted_directness`, and `worst_directness_routes`
    pub fn describe(
        self,
        map: &MapModel,
        out: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<()> {
        let mut count_by_infra: EnumMap<InfraType, usize> = EnumMap::default();
        let mut count_by_los: EnumMap<LevelOfService, usize> = EnumMap::default();
        let mut count_by_tier: EnumMap<Tier, usize> = EnumMap::default();
        let mut count_not_on_network = 0;
        let mut total_count = 0;

        for (r, count) in self.counts {
            total_count += count;
            if let Some(infra_type) = map.infra_types[r.0] {
                count_by_infra[infra_type] += count;
                count_by_tier[map.tiers[r.0].unwrap()] += count;
            } else {
                count_not_on_network += count;
            }
            count_by_los[map.los[r.0]] += count;
        }

        let mut od_percents_infra_type = serde_json::Map::new();
        od_percents_infra_type.insert(
            "Not on the network".to_string(),
            percent(count_not_on_network, total_count).into(),
        );
        for (infra_type, count) in count_by_infra {
            od_percents_infra_type.insert(
                format!("{infra_type:?}"),
                percent(count, total_count).into(),
            );
        }

        let mut od_percents_tier = serde_json::Map::new();
        od_percents_tier.insert(
            "Not on the network".to_string(),
            percent(count_not_on_network, total_count).into(),
        );
        for (tier, count) in count_by_tier {
            od_percents_tier.insert(format!("{tier:?}"), percent(count, total_count).into());
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
            "od_percents_tier".to_string(),
            serde_json::Value::Object(od_percents_tier),
        );
        out.insert(
            "od_percents_los".to_string(),
            serde_json::Value::Object(od_percents_los),
        );
        out.insert(
            "average_weighted_directness".to_string(),
            self.average_weighted_directness.into(),
        );
        out.insert(
            "worst_directness_routes".to_string(),
            serde_json::to_value(&self.worst_directness_routes)?,
        );

        Ok(())
    }
}

impl MapModel {
    pub fn od_counts(&self, fast_sample: bool) -> Result<CountsOD> {
        let keep_directness_routes = 10;
        let direct_profile = self.graph.profile_names["bicycle_direct"];

        let mut rng = WyRand::new_seed(42);

        let mut counts = HashMap::new();
        let mut succeeded = 0;
        let mut failed = 0;
        let mut sum_directness = 0.0;
        let mut sum_count = 0.0;

        let mut worst_directness_routes = Vec::new();

        info!("Evaluating {} desire lines", self.desire_lines.len());

        for (zone1, zone2, raw_count) in &self.desire_lines {
            let (iterations, uptake_multiplier) = if fast_sample {
                (1, *raw_count as f64)
            } else {
                (*raw_count, 1.0)
            };

            for _ in 0..iterations {
                let pt1 = self.od_zones[zone1].random_point(&mut rng);
                let pt2 = self.od_zones[zone2].random_point(&mut rng);

                let start = self.graph.snap_to_road(pt1, direct_profile);
                let end = self.graph.snap_to_road(pt2, direct_profile);
                let Ok(route) = self.graph.routers[direct_profile.0].route(&self.graph, start, end)
                else {
                    failed += 1;
                    continue;
                };
                succeeded += 1;

                let quiet_length = {
                    // Compare with the quiet bike route
                    let quiet_profile = self.graph.profile_names["bicycle_quiet"];
                    if let Ok(quiet_route) =
                        self.graph.routers[quiet_profile.0].route(&self.graph, start, end)
                    {
                        let mut quiet_length = 0.0;
                        for step in &quiet_route.steps {
                            if let PathStep::Road { road, .. } = step {
                                quiet_length += self.graph.roads[road.0].length_meters;
                            }
                        }
                        quiet_length
                    } else {
                        // Skip this one
                        0.0
                    }
                };

                // TODO route.linestring() is more accurate, but slower, and then we have to do the
                // same for comparisons
                let mut route_length = 0.0;
                for step in &route.steps {
                    if let PathStep::Road { road, .. } = step {
                        route_length += self.graph.roads[road.0].length_meters;
                    }
                }

                let count = uptake::pct_godutch_2020(route_length) * uptake_multiplier;

                for step in route.steps {
                    if let PathStep::Road { road, .. } = step {
                        *counts.entry(road).or_insert(0.0) += count;
                    }
                }

                if quiet_length > 0.0 {
                    let directness = quiet_length / route_length;
                    sum_directness += count * directness;
                    sum_count += count;

                    if worst_directness_routes.len() < keep_directness_routes {
                        worst_directness_routes.push((pt1, pt2, directness));
                        worst_directness_routes.sort_by_key(|(_, _, d)| (*d * -100.0) as isize);
                    } else if worst_directness_routes.last().as_ref().unwrap().2 < directness {
                        worst_directness_routes.pop();
                        worst_directness_routes.push((pt1, pt2, directness));
                        worst_directness_routes.sort_by_key(|(_, _, d)| (*d * -100.0) as isize);
                    }
                }
            }
        }

        Ok(CountsOD {
            // Round count after summing decimals
            counts: counts
                .into_iter()
                .map(|(k, v)| (k, v.round() as usize))
                .collect(),
            succeeded,
            failed,
            average_weighted_directness: sum_directness / (sum_count as f64),
            worst_directness_routes: worst_directness_routes
                .into_iter()
                .map(|(start, end, _)| {
                    (
                        self.graph.mercator.pt_to_wgs84(start),
                        self.graph.mercator.pt_to_wgs84(end),
                    )
                })
                .collect(),
        })
    }

    /// Returns detailed GJ with per-road counts
    pub fn evaluate_od(&self, fast_sample: bool) -> Result<String> {
        let od = self.od_counts(fast_sample)?;

        let mut max_count = 0;
        let mut features = Vec::new();
        for (r, count) in &od.counts {
            max_count = max_count.max(*count);

            let mut f = self
                .graph
                .mercator
                .to_wgs84_gj(&self.graph.roads[r.0].linestring);
            f.set_property("count", *count);
            f.set_property("infra_type", serde_json::to_value(self.get_infra_type(*r))?);
            f.set_property("los", serde_json::to_value(self.los[r.0])?);
            features.push(f);
        }

        let mut foreign_members = serde_json::json!({
            "succeeded": od.succeeded,
            "failed": od.failed,
            "max_count": max_count,
        })
        .as_object()
        .unwrap()
        .clone();
        od.describe(self, &mut foreign_members)?;

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: Some(foreign_members),
        })?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Zone {
    pub mp: MultiPolygon,
    // The bbox, rounded to centimeters, for generate_range to work
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

impl Zone {
    fn random_point(&self, rng: &mut WyRand) -> Coord {
        loop {
            let x = (rng.generate_range(self.x1..=self.x2) as f64) / 100.0;
            let y = (rng.generate_range(self.y1..=self.y2) as f64) / 100.0;
            let pt = Coord { x, y };
            if self.mp.contains(&pt) {
                return pt;
            }
        }
    }

    pub fn parse_zones(
        gj: String,
        boundary_wgs84: &MultiPolygon,
        mercator: &Mercator,
    ) -> Result<HashMap<String, Zone>> {
        let gj: FeatureCollection = gj.parse()?;
        let mut zones = HashMap::new();
        for f in gj.features {
            // TODO Rename in data_prep
            let name = f
                .property("InterZone")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            let mut mp: MultiPolygon =
                if matches!(f.geometry.as_ref().unwrap().value, Value::Polygon(_)) {
                    MultiPolygon(vec![f.try_into()?])
                } else {
                    f.try_into()?
                };

            if !boundary_wgs84.intersects(&mp) {
                continue;
            }

            mercator.to_mercator_in_place(&mut mp);
            let bbox = mp.bounding_rect().unwrap();
            zones.insert(
                name,
                Zone {
                    mp,
                    x1: (bbox.min().x * 100.0) as i64,
                    y1: (bbox.min().y * 100.0) as i64,
                    x2: (bbox.max().x * 100.0) as i64,
                    y2: (bbox.max().y * 100.0) as i64,
                },
            );
        }
        info!("Matched to {} zones", zones.len());
        Ok(zones)
    }
}
