use std::collections::HashSet;

use anyhow::Result;
use geo::{BoundingRect, Contains, Intersects, MultiPolygon, Point, Rect};
use geojson::Feature;
use graph::{Graph, RoadID};
use rstar::AABB;
use serde::{Deserialize, Serialize};
use utils::Mercator;

use crate::utils::Quintiles;

// TODO We can't use geojson::ser::to_feature_collection_string and similar magic, because bincode
// doesn't work with it, and we need to do the CRS transform

#[derive(Serialize, Deserialize)]
pub struct School {
    point: Point,
    kind: String,
    name: String,
    // TODO Fix upstream
    pupils: usize,
    pub road: RoadID,
}

impl School {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.point);
        f.set_property("kind", self.kind.clone());
        f.set_property("name", self.name.clone());
        f.set_property("pupils", self.pupils);
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);
        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let mut schools = Vec::new();
        for x in geojson::de::deserialize_feature_collection_str_to_vec::<SchoolGJ>(gj)? {
            if boundary_wgs84.contains(&x.geometry) {
                let point = graph.mercator.to_mercator(&x.geometry);
                let road = graph
                    .snap_to_road(point.into(), graph.profile_names["bicycle"])
                    .road;
                schools.push(School {
                    point,
                    kind: x.r#type,
                    name: x.name,
                    pupils: x.pupils as usize,
                    road,
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

////

#[derive(Serialize, Deserialize)]
pub struct GPHospital {
    point: Point,
    kind: String,
    name: String,
    pub road: RoadID,
}

impl GPHospital {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.point);
        f.set_property("kind", self.kind.clone());
        f.set_property("name", self.name.clone());
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);
        f
    }

    pub fn from_gj(
        gp_gj: &str,
        hospitals_gj: &str,
        boundary_wgs84: &MultiPolygon,
        graph: &Graph,
    ) -> Result<Vec<Self>> {
        let mut gp_hospitals = Vec::new();
        for (gj, kind) in [(gp_gj, "GP"), (hospitals_gj, "hospital")] {
            for x in geojson::de::deserialize_feature_collection_str_to_vec::<GPHospitalGJ>(gj)? {
                if boundary_wgs84.contains(&x.geometry) {
                    let point = graph.mercator.to_mercator(&x.geometry);
                    let road = graph
                        .snap_to_road(point.into(), graph.profile_names["bicycle"])
                        .road;
                    gp_hospitals.push(GPHospital {
                        point,
                        kind: kind.to_string(),
                        name: x.name,
                        road,
                    });
                }
            }
        }
        info!("Matched {} GPs/hospitals", gp_hospitals.len());
        Ok(gp_hospitals)
    }
}

#[derive(Deserialize)]
struct GPHospitalGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: Point,
    name: String,
}

////

#[derive(Serialize, Deserialize)]
pub struct TownCentre {
    polygon: MultiPolygon,
    name: Option<String>,
    pub roads: HashSet<RoadID>,
}

impl TownCentre {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.polygon);
        f.set_property("name", self.name.clone());
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);
        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let mut town_centres = Vec::new();
        for x in geojson::de::deserialize_feature_collection_str_to_vec::<TownCentreGJ>(gj)? {
            if boundary_wgs84.intersects(&x.geometry) {
                let polygon = graph.mercator.to_mercator(&x.geometry);

                // All intersecting roads
                // TODO Could rtree to speed up
                let mut roads: HashSet<RoadID> = HashSet::new();
                for (idx, road) in graph.roads.iter().enumerate() {
                    if polygon.intersects(&road.linestring) {
                        roads.insert(RoadID(idx));
                    }
                }

                town_centres.push(TownCentre {
                    polygon,
                    name: x.name,
                    roads,
                });
            }
        }
        info!("Matched {} town centres", town_centres.len());
        Ok(town_centres)
    }
}

#[derive(Deserialize)]
struct TownCentreGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: MultiPolygon,
    name: Option<String>,
}

////

#[derive(Serialize, Deserialize)]
pub struct Settlement {
    polygon: MultiPolygon,
    name: Option<String>,
    population: usize,
    pub roads: HashSet<RoadID>,
}

impl Settlement {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.polygon);
        f.set_property("name", self.name.clone());
        f.set_property("population", self.population);
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);
        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let mut settlements = Vec::new();
        for x in geojson::de::deserialize_feature_collection_str_to_vec::<SettlementGJ>(gj)? {
            if boundary_wgs84.intersects(&x.geometry) {
                let polygon = graph.mercator.to_mercator(&x.geometry);

                // All intersecting roads
                // TODO Could rtree to speed up
                let mut roads: HashSet<RoadID> = HashSet::new();
                for (idx, road) in graph.roads.iter().enumerate() {
                    if polygon.intersects(&road.linestring) {
                        roads.insert(RoadID(idx));
                    }
                }

                settlements.push(Settlement {
                    polygon,
                    name: x.name,
                    population: x.population as usize,
                    roads,
                });
            }
        }
        info!("Matched {} settlements", settlements.len());
        Ok(settlements)
    }
}

#[derive(Deserialize)]
struct SettlementGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: MultiPolygon,
    name: Option<String>,
    #[serde(rename = "Population")]
    population: f64,
}

////

#[derive(Serialize, Deserialize)]
pub struct DataZone {
    polygon: MultiPolygon,
    id: String,
    imd_rank: usize,
    pub imd_percentile: usize,
    pub population: usize,
    pub roads: HashSet<RoadID>,
    area_km2: f64,
    // Relative to the study area, not all of Scotland
    density_quintile: usize,
}

impl DataZone {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.polygon);
        f.set_property("id", self.id.clone());
        f.set_property("imd_rank", self.imd_rank);
        f.set_property("imd_percentile", self.imd_percentile);
        f.set_property("population", self.population);
        f.set_property("area_km2", self.area_km2);
        f.set_property("density_quintile", self.density_quintile);
        f.set_property("reachable", reachable);
        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let profile = graph.profile_names["bicycle"];

        let mut zones = Vec::new();
        let mut densities = Vec::new();
        for x in geojson::de::deserialize_feature_collection_str_to_vec::<DataZoneGJ>(gj)? {
            if boundary_wgs84.intersects(&x.geometry) {
                let polygon = graph.mercator.to_mercator(&x.geometry);

                // TODO rstar can't directly calculate a MultiPolygon envelope
                let bbox: Rect = polygon.bounding_rect().unwrap().into();
                let envelope = AABB::from_corners(
                    Point::new(bbox.min().x, bbox.min().y),
                    Point::new(bbox.max().x, bbox.max().y),
                );

                // All intersecting roads
                let roads: HashSet<RoadID> = graph.routers[profile.0]
                    .closest_road
                    .locate_in_envelope_intersecting(&envelope)
                    .map(|obj| obj.data)
                    .collect();

                let area_km2 = x.area / 10.0e6;
                zones.push(DataZone {
                    polygon,
                    id: x.id,
                    imd_rank: x.rank,
                    imd_percentile: x.percentile,
                    population: x.population,
                    area_km2,
                    roads,
                    density_quintile: 0,
                });

                densities.push(((x.population as f64) / area_km2) as usize);
            }
        }

        let stats = Quintiles::new(&densities);
        for zone in &mut zones {
            zone.density_quintile =
                stats.quintile(((zone.population as f64) / zone.area_km2) as usize);
        }

        info!("Matched {} data zones", zones.len());
        Ok(zones)
    }
}

#[derive(Deserialize)]
struct DataZoneGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: MultiPolygon,
    #[serde(rename = "DataZone")]
    id: String,
    rank: usize,
    percentile: usize,
    population: usize,
    area: f64,
}
