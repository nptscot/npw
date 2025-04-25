use std::collections::HashMap;

use anyhow::Result;
use enum_map::EnumMap;
use geo::{BoundingRect, Centroid, Contains, Coord, Distance, Euclidean, Intersects, MultiPolygon};
use geojson::{FeatureCollection, Value};
use graph::{PathStep, RoadID, Timer};
use itertools::Itertools;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use utils::Mercator;

use crate::{
    stats::percent, uptake, utils::into_object_value, InfraType, LevelOfService, MapModel, Tier,
};

pub struct CountsOD {
    pub counts: HashMap<RoadID, usize>,
    pub succeeded: usize,
    pub failed: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SlowStats {
    pub average_weighted_directness: f64,
    pub worst_directness_routes: Vec<(Coord, Coord)>,
}

impl CountsOD {
    /// Populate `out` with `od_percents_los`, `od_percents_infra_type`, and `od_percents_tier`
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

        Ok(())
    }
}

impl MapModel {
    pub fn od_counts(&self, fast_sample: bool, profile_name: &str) -> Result<CountsOD> {
        if profile_name == "bicycle_quiet" {
            assert!(self.quiet_router_ok);
        }
        let profile = self.graph.profile_names[profile_name];

        let mut rng = WyRand::new_seed(42);

        let mut counts = HashMap::new();
        let mut succeeded = 0;
        let mut failed = 0;

        info!("Evaluating {} desire lines", self.desire_lines.len());

        for (zone1, zone2, raw_count) in &self.desire_lines {
            let (iterations, uptake_multiplier) = if fast_sample {
                (1, *raw_count as f64)
            } else {
                (*raw_count, 1.0)
            };

            for _ in 0..iterations {
                let input_pt1 = self.od_zones[zone1].random_point(&mut rng);
                let input_pt2 = self.od_zones[zone2].random_point(&mut rng);

                let start = self.graph.snap_to_road(input_pt1, profile);
                let end = self.graph.snap_to_road(input_pt2, profile);
                let Ok(route) = self.graph.routers[profile.0].route(&self.graph, start, end) else {
                    failed += 1;
                    continue;
                };
                succeeded += 1;

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
        })
    }

    /// Returns detailed GJ with per-road counts
    pub fn evaluate_od(&self, fast_sample: bool) -> Result<String> {
        let od = self.od_counts(fast_sample, "bicycle_quiet")?;

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

        let mut foreign_members = into_object_value(serde_json::json!({
            "succeeded": od.succeeded,
            "failed": od.failed,
            "max_count": max_count,
        }));
        od.describe(self, &mut foreign_members)?;

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: Some(foreign_members),
        })?)
    }

    pub fn get_slow_stats(&self, timer: &mut Timer) -> SlowStats {
        assert!(self.quiet_router_ok);

        timer.step("generate OD pairs");
        let requests = self.get_town_centre_od();

        timer.step(format!("calculate {} routes", requests.len()));
        let keep_directness_routes = 10;
        let quiet_profile = self.graph.profile_names["bicycle_quiet"];

        let mut sum_directness = 0.0;
        let mut sum_count = 0;
        let mut worst_directness_routes = Vec::new();

        for (input_pt1, input_pt2) in requests {
            let start = self.graph.snap_to_road(input_pt1, quiet_profile);
            let end = self.graph.snap_to_road(input_pt2, quiet_profile);
            let Ok(route) = self.graph.routers[quiet_profile.0].route(&self.graph, start, end)
            else {
                continue;
            };

            // route.linestring() is more accurate, but slower
            let mut route_length = 0.0;
            let mut snapped_pt1 = None;
            let mut snapped_pt2 = None;
            for (pos, step) in route.steps.iter().with_position() {
                if let PathStep::Road { road, forwards } = step {
                    let road = &self.graph.roads[road.0];
                    route_length += road.length_meters;

                    match pos {
                        itertools::Position::First => {
                            snapped_pt1 = Some(if *forwards {
                                road.linestring.0[0]
                            } else {
                                *road.linestring.0.last().unwrap()
                            });
                        }
                        itertools::Position::Last => {
                            snapped_pt2 = Some(if !*forwards {
                                road.linestring.0[0]
                            } else {
                                *road.linestring.0.last().unwrap()
                            });
                        }
                        itertools::Position::Only => {
                            snapped_pt1 = Some(if *forwards {
                                road.linestring.0[0]
                            } else {
                                *road.linestring.0.last().unwrap()
                            });
                            snapped_pt2 = Some(if !*forwards {
                                road.linestring.0[0]
                            } else {
                                *road.linestring.0.last().unwrap()
                            });
                        }
                        itertools::Position::Middle => {}
                    }
                }
            }

            let straight_line_length =
                Euclidean.distance(snapped_pt1.unwrap(), snapped_pt2.unwrap());
            if straight_line_length == 0.0 {
                // This should never happen in practice; this was a degenerately empty route
                continue;
            }

            let directness = route_length / straight_line_length;
            sum_directness += directness;
            sum_count += 1;

            if worst_directness_routes.len() < keep_directness_routes {
                worst_directness_routes.push((input_pt1, input_pt2, directness));
                worst_directness_routes.sort_by_key(|(_, _, d)| (*d * -100.0) as isize);
            } else if worst_directness_routes.last().as_ref().unwrap().2 < directness {
                worst_directness_routes.pop();
                worst_directness_routes.push((input_pt1, input_pt2, directness));
                worst_directness_routes.sort_by_key(|(_, _, d)| (*d * -100.0) as isize);
            }
        }

        SlowStats {
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
        }
    }

    /// Get straight lines between all pairs of town centres less than 5km
    fn get_town_centre_od(&self) -> Vec<(Coord, Coord)> {
        let mut requests = Vec::new();
        for (idx1, tc1) in self.town_centres.iter().enumerate() {
            for (idx2, tc2) in self.town_centres.iter().enumerate() {
                // Routes are bidirectional, so just check one direction
                if idx1 >= idx2 {
                    continue;
                }

                let centroid1 = tc1.polygon.centroid().unwrap();
                let centroid2 = tc2.polygon.centroid().unwrap();
                let dist = Euclidean.distance(centroid1, centroid2);

                if dist > 5000.0 {
                    continue;
                }

                requests.push((centroid1.into(), centroid2.into()));
            }
        }
        requests
    }

    pub fn precalculate_demands(&mut self) -> Result<()> {
        assert!(self.precalculated_demands.is_empty());

        let fast_sample = false;
        let counts = self.od_counts(fast_sample, "bicycle_direct")?;
        for idx in 0..self.graph.roads.len() {
            self.precalculated_demands
                .push(counts.counts.get(&RoadID(idx)).cloned().unwrap_or(0));
        }

        let (high_demand_threshold, medium_demand_threshold) =
            find_cycling_demand_thresholds(&self.precalculated_demands)?;
        self.high_demand_threshold = high_demand_threshold;
        self.medium_demand_threshold = medium_demand_threshold;
        Ok(())
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

fn find_cycling_demand_thresholds(demands: &Vec<usize>) -> Result<(usize, usize)> {
    info!("Calculating ckmeans for {} values", demands.len());
    let num_classes = 10;
    let results = ckmeans::ckmeans(demands, num_classes)?;
    let maxes: Vec<usize> = results
        .into_iter()
        .map(|group| *group.last().unwrap())
        .collect();
    // 5th highest break for high, 7th highest break for medium
    let high = maxes[10 - 5];
    let medium = maxes[10 - 7];
    info!("ckmeans classes are {maxes:?}. high_demand_threshold is {high}, medium_demand_threshold is {medium}");
    Ok((high, medium))
}
