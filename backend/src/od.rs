use std::collections::HashMap;

use anyhow::Result;
use geo::{BoundingRect, Contains, Coord, MultiPolygon};
use geojson::{FeatureCollection, Value};
use nanorand::{Rng, WyRand};
use utils::Mercator;

use crate::MapModel;

impl MapModel {
    pub fn evaluate_od(&self, gj: String, od: Vec<(String, String, usize)>) -> Result<String> {
        let zones = parse_zones(gj, &self.graph.mercator)?;
        let mut rng = WyRand::new_seed(42);

        let mut success = 0;
        for (zone1, zone2, count) in od {
            let from = match zones.get(&zone1) {
                Some(zone) => zone.random_point(&mut rng),
                None => {
                    continue;
                }
            };
            let to = match zones.get(&zone2) {
                Some(zone) => zone.random_point(&mut rng),
                None => {
                    continue;
                }
            };

            // TODO Route
            success += 1;
        }

        Ok(format!("{success} succeeded"))
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
