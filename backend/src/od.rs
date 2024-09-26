use std::collections::HashMap;

use anyhow::Result;
use geo::{BoundingRect, Contains, Coord, MultiPolygon};
use geojson::{Feature, FeatureCollection, Geometry, Value};
use graph::{Mode, PathStep};
use nanorand::{Rng, WyRand};
use utils::Mercator;

use crate::{InfraType, MapModel};

impl MapModel {
    pub fn evaluate_od(&self, gj: String, od: Vec<(String, String, usize)>) -> Result<String> {
        let zones = parse_zones(gj, &self.graph.mercator)?;
        let mut rng = WyRand::new_seed(42);
        let infra_types = self.get_infra_types();

        let mut counts = HashMap::new();
        let mut succeeded = 0;
        let mut failed = 0;

        for (zone1, zone2, count) in od {
            for _ in 0..count {
                let pt1 = match zones.get(&zone1) {
                    Some(zone) => zone.random_point(&mut rng),
                    None => {
                        continue;
                    }
                };
                let pt2 = match zones.get(&zone2) {
                    Some(zone) => zone.random_point(&mut rng),
                    None => {
                        continue;
                    }
                };

                let mode = Mode::Bicycle;
                let start = self.graph.snap_to_road(pt1, mode);
                let end = self.graph.snap_to_road(pt2, mode);
                let Ok(route) = self.graph.router[mode].route(&self.graph, start, end) else {
                    failed += 1;
                    continue;
                };
                succeeded += 1;

                // TODO Use a lower-level API to squeeze out some speed
                for step in route.steps {
                    if let PathStep::Road { road, .. } = step {
                        *counts.entry(road).or_insert(0) += 1;
                    }
                }
            }
        }

        let mut max_count = 0;
        let mut features = Vec::new();
        for (r, count) in counts {
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
                serde_json::to_value(
                    infra_types
                        .get(&r)
                        .cloned()
                        .unwrap_or(InfraType::MixedTraffic),
                )
                .unwrap(),
            );
            features.push(f);
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: Some(
                serde_json::json!({
                    "succeeded": succeeded,
                    "failed": failed,
                    "max_count": max_count,
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
        })?)
    }
}

struct Zone {
    mp: MultiPolygon,
    // The bbox, rounded to centimeters, for generate_range to work
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
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
}

// TODO Clip to study area
fn parse_zones(gj: String, mercator: &Mercator) -> Result<HashMap<String, Zone>> {
    let gj: FeatureCollection = gj.parse()?;
    let mut zones = HashMap::new();
    for f in gj.features {
        let name = f.property("name").unwrap().as_str().unwrap().to_string();
        let mut mp: MultiPolygon =
            if matches!(f.geometry.as_ref().unwrap().value, Value::Polygon(_)) {
                MultiPolygon(vec![f.try_into()?])
            } else {
                f.try_into()?
            };
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
    Ok(zones)
}
