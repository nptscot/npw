use std::collections::{HashMap, HashSet};

use anyhow::Result;
use enum_map::EnumMap;
use geo::{Centroid, Coord, Distance, Euclidean};
use geojson::FeatureCollection;
use graph::{Graph, PathStep, RoadID, Route, Timer};
use nanorand::WyRand;
use serde::{Deserialize, Serialize};

use crate::{
    stats::percent, uptake, utils::into_object_value, InfraType, LevelOfService, MapModel, Tier,
};

pub struct CountsOD {
    pub counts: HashMap<RoadID, usize>,
    pub succeeded: usize,
    pub failed: usize,
}

#[derive(Default, Serialize, Deserialize)]
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

        let mut counts = HashMap::new();
        let mut succeeded = 0;
        let mut failed = 0;

        let (requests, total_trips) = self.get_od_requests(fast_sample);

        info!(
            "Evaluating {} desire lines, representing {total_trips} trips",
            requests.len()
        );
        let mut total_uptake = 0.0;

        for (pt1, pt2, uptake_multiplier) in requests {
            let start = self.graph.snap_to_road(pt1, profile);
            let end = self.graph.snap_to_road(pt2, profile);
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
            total_uptake += count;
            for step in route.steps {
                if let PathStep::Road { road, .. } = step {
                    *counts.entry(road).or_insert(0.0) += count;
                }
            }
        }

        info!("Total uptake {}", total_uptake.round());

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

    fn get_od_requests(&self, fast_sample: bool) -> (Vec<(Coord, Coord, f64)>, usize) {
        let mut rng = WyRand::new_seed(42);
        let mut total_trips = 0;
        let mut requests = Vec::new();

        for (zone1, zone2, raw_count) in &self.commute_desire_lines {
            total_trips += *raw_count;
            let (iterations, uptake_multiplier) = if fast_sample {
                (1, *raw_count as f64)
            } else {
                (*raw_count, 1.0)
            };

            for _ in 0..iterations {
                let pt1 = self.data_zones[*zone1].random_point(&mut rng);
                let pt2 = self.data_zones[*zone2].random_point(&mut rng);
                requests.push((pt1, pt2, uptake_multiplier));
            }
        }

        for (zone1, pt2, raw_count) in &self.utility_desire_lines {
            total_trips += *raw_count;
            let (iterations, uptake_multiplier) = if fast_sample {
                (1, *raw_count as f64)
            } else {
                (*raw_count, 1.0)
            };

            for _ in 0..iterations {
                let pt1 = self.data_zones[*zone1].random_point(&mut rng);
                requests.push((pt1, *pt2, uptake_multiplier));
            }
        }

        (requests, total_trips)
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

        // Edge case for Orkney Islands
        if requests.is_empty() {
            return SlowStats {
                average_weighted_directness: 1.0,
                worst_directness_routes: Vec::new(),
            };
        }

        timer.step(format!("calculate {} routes", requests.len()));
        let keep_directness_routes = 10;
        let quiet_profile = self.graph.profile_names["bicycle_quiet"];
        let direct_profile = self.graph.profile_names["bicycle_direct"];

        let mut sum_directness = 0.0;
        let mut sum_count = 0;
        let mut worst_directness_routes = Vec::new();

        for (input_pt1, input_pt2) in requests {
            let start = self.graph.snap_to_road(input_pt1, quiet_profile);
            let end = self.graph.snap_to_road(input_pt2, quiet_profile);
            let Ok(quiet_route) =
                self.graph.routers[quiet_profile.0].route(&self.graph, start, end)
            else {
                continue;
            };
            let Ok(direct_route) =
                self.graph.routers[direct_profile.0].route(&self.graph, start, end)
            else {
                continue;
            };

            let directness = route_length_fast(&self.graph, &quiet_route)
                / route_length_fast(&self.graph, &direct_route);
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

    pub fn get_town_centre_routes(&self) -> Result<String> {
        assert!(self.quiet_router_ok);
        let quiet_profile = self.graph.profile_names["bicycle_quiet"];
        let direct_profile = self.graph.profile_names["bicycle_direct"];
        let mut quiet_roads = HashSet::new();
        let mut direct_roads = HashSet::new();

        for (input_pt1, input_pt2) in self.get_town_centre_od() {
            let start = self.graph.snap_to_road(input_pt1, quiet_profile);
            let end = self.graph.snap_to_road(input_pt2, quiet_profile);

            if let Ok(quiet_route) =
                self.graph.routers[quiet_profile.0].route(&self.graph, start, end)
            {
                for step in quiet_route.steps {
                    if let PathStep::Road { road, .. } = step {
                        quiet_roads.insert(road);
                    }
                }
            }

            if let Ok(direct_route) =
                self.graph.routers[direct_profile.0].route(&self.graph, start, end)
            {
                for step in direct_route.steps {
                    if let PathStep::Road { road, .. } = step {
                        direct_roads.insert(road);
                    }
                }
            }
        }

        let mut features = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let direct = direct_roads.contains(&RoadID(idx));
            let quiet = quiet_roads.contains(&RoadID(idx));
            if direct || quiet {
                let mut f = self.graph.mercator.to_wgs84_gj(&road.linestring);
                f.set_property("direct", direct);
                f.set_property("quiet", quiet);
                f.set_property("los", serde_json::to_value(&self.los[idx])?);
                features.push(f);
            }
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
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

// route.linestring() is more accurate, but slower
fn route_length_fast(graph: &Graph, route: &Route) -> f64 {
    let mut len = 0.0;
    for step in &route.steps {
        if let PathStep::Road { road, .. } = step {
            len += graph.roads[road.0].length_meters;
        }
    }
    len
}
