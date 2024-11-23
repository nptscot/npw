use std::collections::HashSet;

use anyhow::Result;
use geo::{Contains, Intersects, MultiPolygon, Point};
use geojson::Feature;
use graph::{Graph, RoadID};
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
    pub road: RoadID,
}

impl School {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.point);
        f.set_property("kind", self.kind.clone());
        f.set_property("name", self.name.clone());
        f.set_property("pupils", self.pupils);
        f.set_property("reachable", reachable);
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

#[derive(Serialize, Deserialize)]
pub struct GPHospital {
    point: Point,
    kind: String,
    name: String,
    pub road: RoadID,
}

impl GPHospital {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.point);
        f.set_property("kind", self.kind.clone());
        f.set_property("name", self.name.clone());
        f.set_property("reachable", reachable);
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

#[derive(Serialize, Deserialize)]
pub struct TownCentre {
    polygon: MultiPolygon,
    name: Option<String>,
    pub roads: HashSet<RoadID>,
}

impl TownCentre {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.polygon);
        f.set_property("name", self.name.clone());
        f.set_property("reachable", reachable);
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

#[derive(Serialize, Deserialize)]
pub struct IMDZone {
    polygon: MultiPolygon,
    id: String,
    rank: usize,
    percentile: usize,
    pub population: usize,
    pub roads: HashSet<RoadID>,
}

impl IMDZone {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.polygon);
        f.set_property("id", self.id.clone());
        f.set_property("rank", self.rank);
        f.set_property("percentile", self.percentile);
        f.set_property("population", self.population);
        f.set_property("reachable", reachable);
        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let mut zones = Vec::new();
        for x in geojson::de::deserialize_feature_collection_str_to_vec::<IMDZoneGJ>(gj)? {
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

                zones.push(IMDZone {
                    polygon,
                    id: x.id,
                    rank: x.rank,
                    percentile: x.percentile,
                    population: x.population,
                    roads,
                });
            }
        }
        info!("Matched {} IMD zones", zones.len());
        Ok(zones)
    }
}

#[derive(Deserialize)]
struct IMDZoneGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: MultiPolygon,
    #[serde(rename = "DataZone")]
    id: String,
    rank: usize,
    percentile: usize,
    population: usize,
}
