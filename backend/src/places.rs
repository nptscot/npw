use anyhow::Result;
use geo::{Contains, MultiPolygon, Point};
use geojson::{Feature, Geometry};
use serde::{Deserialize, Serialize};
use utils::Mercator;

// TODO We can't use geojson::ser::to_feature_collection_string and similar magic, because bincode
// doesn't work with it, and we need to do the CRS transform

#[derive(Serialize, Deserialize)]
pub struct School {
    point: Point,
    kind: String,
    name: String,
    // TODO Fix upstream
    pupils: usize,
}

impl School {
    pub fn to_gj(&self, mercator: &Mercator) -> Feature {
        let mut f = Feature::from(Geometry::from(&mercator.to_wgs84(&self.point)));
        f.set_property("kind", self.kind.clone());
        f.set_property("name", self.name.clone());
        f.set_property("pupils", self.pupils);
        f
    }

    pub fn from_gj(
        gj: &str,
        boundary_wgs84: &MultiPolygon,
        mercator: &Mercator,
    ) -> Result<Vec<Self>> {
        let mut schools = Vec::new();
        for s in geojson::de::deserialize_feature_collection_str_to_vec::<SchoolGJ>(gj)? {
            if boundary_wgs84.contains(&s.geometry) {
                schools.push(School {
                    point: mercator.to_mercator(&s.geometry),
                    kind: s.r#type,
                    name: s.name,
                    pupils: s.pupils as usize,
                });
            }
        }
        info!("Matched {} schools", schools.len());
        Ok(schools)
    }
}

#[derive(Deserialize)]
struct SchoolGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: Point,
    r#type: String,
    name: String,
    pupils: f64,
}
