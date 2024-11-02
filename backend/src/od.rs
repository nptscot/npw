use std::collections::HashMap;

use anyhow::Result;
use enum_map::EnumMap;
use geo::{BoundingRect, Contains, Coord, Distance, Euclidean, Intersects, MultiPolygon};
use geojson::{Feature, FeatureCollection, Geometry, Value};
use graph::{PathStep, RoadID};
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use utils::Mercator;

use crate::{InfraType, MapModel};

pub struct CountsOD {
    pub counts: HashMap<RoadID, usize>,
    pub succeeded: usize,
    pub failed: usize,
    pub average_weighted_directness: f64,
}

impl MapModel {
    pub fn od_counts(&self) -> Result<CountsOD> {
        let mut rng = WyRand::new_seed(42);

        let mut counts = HashMap::new();
        let mut succeeded = 0;
        let mut failed = 0;
        let mut sum_directness = 0.0;
        let mut sum_count = 0;

        info!("Evaluating {} desire lines", self.desire_lines.len());

        // TODO TEMPORARILY, evaluate just one route from each desire line and weight it by the
        // count
        for (zone1, zone2, count) in &self.desire_lines {
            let pt1 = self.zones[zone1].random_point(&mut rng);
            let pt2 = self.zones[zone2].random_point(&mut rng);

            let profile = self.graph.profile_names["bicycle"];
            let start = self.graph.snap_to_road(pt1, profile);
            let end = self.graph.snap_to_road(pt2, profile);
            let Ok(route) = self.graph.routers[profile.0].route(&self.graph, start, end) else {
                failed += 1;
                continue;
            };
            succeeded += 1;

            // route.linestring() is more accurate, but slower, and then harder to find the snapped
            // position on the road for direct_length
            let mut route_length = 0.0;
            let direct_length = Euclidean::distance(
                self.graph.intersections[start.intersection.0].point,
                self.graph.intersections[end.intersection.0].point,
            );

            // TODO Use a lower-level API to squeeze out some speed
            for step in route.steps {
                if let PathStep::Road { road, .. } = step {
                    *counts.entry(road).or_insert(0) += *count;
                    route_length += self.graph.roads[road.0].length_meters;
                }
            }

            if direct_length > 0.0 {
                let directness = route_length / direct_length;
                sum_directness += (*count as f64) * directness;
                sum_count += *count;
            }
        }

        Ok(CountsOD {
            counts,
            succeeded,
            failed,
            average_weighted_directness: sum_directness / (sum_count as f64),
        })
    }

    /// Returns detailed GJ with per-road counts
    pub fn evaluate_od(&self) -> Result<String> {
        let out = self.od_counts()?;

        let mut count_by_infra: EnumMap<InfraType, usize> = EnumMap::default();
        let mut count_off_network = 0;
        let mut total_count = 0;

        let mut max_count = 0;
        let mut features = Vec::new();
        for (r, count) in out.counts {
            max_count = max_count.max(count);
            let mut f = Feature::from(Geometry::from(
                &self
                    .graph
                    .mercator
                    .to_wgs84(&self.graph.roads[r.0].linestring),
            ));
            f.set_property("count", count);
            f.set_property(
                "infra_type",
                serde_json::to_value(self.get_infra_type(r)).unwrap(),
            );
            features.push(f);

            total_count += count;
            if let Some(infra_type) = self.infra_types[r.0] {
                count_by_infra[infra_type] += count;
            } else {
                count_off_network += count;
            }
        }

        let mut foreign_members = serde_json::json!({
            "succeeded": out.succeeded,
            "failed": out.failed,
            "max_count": max_count,
            "percent_off_network": percent(count_off_network, total_count),
        })
        .as_object()
        .unwrap()
        .clone();
        let mut percent_on_network = serde_json::Map::new();
        for (infra_type, count) in count_by_infra {
            percent_on_network.insert(
                format!("{infra_type:?}"),
                percent(count, total_count).into(),
            );
        }
        foreign_members.insert(
            "percent_on_network".to_string(),
            serde_json::Value::Object(percent_on_network),
        );

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

fn percent(x: usize, total: usize) -> f64 {
    if total == 0 {
        0.0
    } else {
        (x as f64) / (total as f64)
    }
}
