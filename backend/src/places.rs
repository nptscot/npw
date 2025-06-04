use std::collections::HashSet;

use anyhow::Result;
use geo::{Area, BooleanOps, Centroid, Contains, Coord, Intersects, MultiPolygon, Point, Polygon};
use geojson::Feature;
use graph::{Graph, RoadID};
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use utils::Mercator;

// TODO We can't use geojson::ser::to_feature_collection_string and similar magic, because bincode
// doesn't work with it, and we need to do the CRS transform

#[derive(Serialize, Deserialize)]
pub struct School {
    pub point: Point,
    kind: String,
    name: String,
    // TODO Fix upstream
    pupils: usize,
    pub road: RoadID,
    sort: f64,
}

impl School {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.point);
        f.set_property("poi_kind", "schools");
        f.set_property("kind", self.kind.clone());
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);
        f.set_property("sort", self.sort);

        let name = if self.name.is_empty() {
            "This school"
        } else {
            &self.name
        };
        f.set_property(
            "description",
            format!(
                "{name} (a {} school with {} pupils)",
                self.kind, self.pupils
            ),
        );

        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let mut schools = Vec::new();
        for obj in geojson::de::deserialize_feature_collection_str_to_vec::<SchoolGJ>(gj)? {
            if boundary_wgs84.contains(&obj.geometry) {
                let point = graph.mercator.to_mercator(&obj.geometry);
                let road = graph
                    .snap_to_road(point.into(), graph.profile_names["bicycle_direct"])
                    .road;
                let x = point.x() / graph.mercator.width;
                let y = point.y() / graph.mercator.height;
                let sort =
                    hilbert_2d::xy2h_continuous_f64(x, y, hilbert_2d::Variant::Hilbert) * 1_000.0;

                schools.push(School {
                    point,
                    kind: obj.r#type,
                    name: obj.name,
                    pupils: obj.pupils as usize,
                    road,
                    sort,
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
    pub point: Point,
    kind: String,
    name: String,
    pub road: RoadID,
    sort: f64,
}

impl GPHospital {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.point);
        f.set_property("poi_kind", "gp_hospitals");
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);
        f.set_property("sort", self.sort);

        if self.name.is_empty() {
            f.set_property("description", format!("This {}", self.kind));
        } else {
            f.set_property("description", self.name.clone());
        }

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
            for obj in geojson::de::deserialize_feature_collection_str_to_vec::<GPHospitalGJ>(gj)? {
                if boundary_wgs84.contains(&obj.geometry) {
                    let point = graph.mercator.to_mercator(&obj.geometry);
                    let road = graph
                        .snap_to_road(point.into(), graph.profile_names["bicycle_direct"])
                        .road;
                    let x = point.x() / graph.mercator.width;
                    let y = point.y() / graph.mercator.height;
                    let sort = hilbert_2d::xy2h_continuous_f64(x, y, hilbert_2d::Variant::Hilbert)
                        * 1_000.0;

                    gp_hospitals.push(GPHospital {
                        point,
                        kind: kind.to_string(),
                        name: obj.name,
                        road,
                        sort,
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

///

#[derive(Serialize, Deserialize)]
pub struct RailwayStation {
    pub point: Point,
    name: Option<String>,
    pub road: RoadID,
    sort: f64,
}

impl RailwayStation {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.point);
        f.set_property("poi_kind", "railway_stations");
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);
        f.set_property("sort", self.sort);

        f.set_property(
            "description",
            self.name
                .clone()
                .unwrap_or_else(|| "This railway station".to_string()),
        );

        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let mut railway_stations = Vec::new();
        for obj in geojson::de::deserialize_feature_collection_str_to_vec::<RailwayStationGJ>(gj)? {
            if boundary_wgs84.contains(&obj.geometry) {
                let point = graph.mercator.to_mercator(&obj.geometry);
                let road = graph
                    .snap_to_road(point.into(), graph.profile_names["bicycle_direct"])
                    .road;
                let x = point.x() / graph.mercator.width;
                let y = point.y() / graph.mercator.height;
                let sort =
                    hilbert_2d::xy2h_continuous_f64(x, y, hilbert_2d::Variant::Hilbert) * 1_000.0;

                railway_stations.push(RailwayStation {
                    point,
                    name: obj.name,
                    road,
                    sort,
                });
            }
        }
        info!("Matched {} railway stations", railway_stations.len());
        Ok(railway_stations)
    }
}

#[derive(Deserialize)]
struct RailwayStationGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: Point,
    name: Option<String>,
}

////

#[derive(Serialize, Deserialize)]
pub struct Greenspace {
    pub polygon: MultiPolygon,
    pub centroid_wgs84: Coord,
    pub name: Option<String>,
    pub access_points: Vec<Point>,
    pub roads: HashSet<RoadID>,
    pub sort: f64,
}

impl Greenspace {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Vec<Feature> {
        let mut features = Vec::new();
        {
            let mut f = mercator.to_wgs84_gj(&self.polygon);
            f.set_property("poi_kind", "greenspaces");
            f.set_property("kind", "greenspace");
            f.set_property(
                "description",
                if let Some(name) = &self.name {
                    name
                } else {
                    "This greenspace"
                },
            );
            f.set_property("reachable", reachable);
            f.set_property("idx", idx);
            f.set_property("sort", self.sort);
            f.set_property(
                "centroid",
                vec![self.centroid_wgs84.x, self.centroid_wgs84.y],
            );
            features.push(f);
        }
        for pt in &self.access_points {
            let mut f = mercator.to_wgs84_gj(pt);
            f.set_property("kind", "access point");
            features.push(f);
        }
        features
    }
}

////

#[derive(Serialize, Deserialize)]
pub struct TownCentre {
    pub polygon: Polygon,
    name: Option<String>,
    pub roads: HashSet<RoadID>,
}

impl TownCentre {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.polygon);
        f.set_property("poi_kind", "town_centres");
        f.set_property("description", self.name.clone().unwrap_or_else(String::new));
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);
        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let boundary_mercator = graph.mercator.to_mercator(boundary_wgs84);

        let mut town_centres = Vec::new();
        for x in geojson::de::deserialize_feature_collection_str_to_vec::<TownCentreGJ>(gj)? {
            if boundary_wgs84.intersects(&x.geometry) {
                let polygon = graph.mercator.to_mercator(&x.geometry);

                // How much of the zone intersects the study area?
                let overlap = boundary_mercator.intersection(&polygon);
                let ratio_in_boundary = overlap.unsigned_area() / polygon.unsigned_area();
                if ratio_in_boundary < 0.1 {
                    info!(
                        "Skipping town centre {:?} because only {}% of it overlaps the boundary",
                        x.name,
                        ratio_in_boundary * 100.0
                    );
                    continue;
                }

                // All intersecting roads
                // TODO Could rtree to speed up
                let mut roads: HashSet<RoadID> = HashSet::new();
                for (idx, road) in graph.roads.iter().enumerate() {
                    if polygon.intersects(&road.linestring) {
                        roads.insert(RoadID(idx));
                    }
                }
                if roads.is_empty() {
                    info!("Town centre {:?} doesn't intersect any road. Just snapping to one arbitrary close road.", x.name);
                    let centroid = polygon.centroid().unwrap();
                    roads.insert(
                        graph
                            .snap_to_road(centroid.into(), graph.profile_names["bicycle_direct"])
                            .road,
                    );
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
    geometry: Polygon,
    name: Option<String>,
}

////

#[derive(Serialize, Deserialize)]
pub struct Settlement {
    pub polygon: Polygon,
    pub name: Option<String>,
    pub population: usize,
    pub roads: HashSet<RoadID>,
}

impl Settlement {
    pub fn to_gj(&self, mercator: &Mercator, reachable: bool, idx: usize) -> Feature {
        let mut f = mercator.to_wgs84_gj(&self.polygon);
        f.set_property("poi_kind", "settlements");
        f.set_property("reachable", reachable);
        f.set_property("idx", idx);

        let name = if let Some(name) = &self.name {
            name
        } else {
            "This settlement"
        };
        f.set_property(
            "description",
            format!("{name} with population {}", self.population),
        );

        f
    }

    pub fn from_gj(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<Self>> {
        let boundary_mercator = graph.mercator.to_mercator(boundary_wgs84);

        let mut settlements = Vec::new();
        for x in geojson::de::deserialize_feature_collection_str_to_vec::<SettlementGJ>(gj)? {
            if boundary_wgs84.intersects(&x.geometry) {
                let settlement_mercator = graph.mercator.to_mercator(&x.geometry);
                // Clip the settlement to the study area
                let mut settlement_pieces = boundary_mercator.intersection(&settlement_mercator);
                settlement_pieces.0.retain(|polygon| {
                    // Settlement polygons are more precise than the simplified local authority
                    // boundaries. Rather than switch to the precise LA boundaries (from
                    // https://data.spatialhub.scot/dataset/local_authority_boundaries-is/resource/d24c5735-0f1c-4819-a6bd-dbfeb93bd8e4)
                    // and incur a file size hit, just check for a minimum area of the clipped settlements. By manual inspection, this threshold is reasonable.
                    let keep = polygon.unsigned_area() >= 10_000.0;
                    if !keep {
                        info!(
                            "Skipping settlement {:?} because it's tiny after being clipped",
                            x.name
                        );
                    }
                    keep
                });

                let n = settlement_pieces.0.len();
                for (idx, polygon) in settlement_pieces.0.into_iter().enumerate() {
                    // All intersecting roads
                    // TODO Could rtree to speed up
                    let mut roads: HashSet<RoadID> = HashSet::new();
                    for (idx, road) in graph.roads.iter().enumerate() {
                        if polygon.intersects(&road.linestring) {
                            roads.insert(RoadID(idx));
                        }
                    }
                    if roads.is_empty() {
                        // Just log it; there are some settlement pieces on empty islands
                        error!("Settlement {:?} doesn't snap to any roads", x.name);
                    }

                    settlements.push(Settlement {
                        polygon,
                        name: if let Some(ref name) = x.name {
                            if n > 1 {
                                Some(format!("{name} ({} / {n})", idx + 1))
                            } else {
                                Some(name.clone())
                            }
                        } else {
                            None
                        },
                        population: x.population as usize,
                        roads,
                    });
                }
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

// These are data zones from the 2020 SIMD
#[derive(Serialize, Deserialize)]
pub struct DataZone {
    pub polygon: MultiPolygon,
    pub id: String,
    pub imd_rank: usize,
    pub imd_percentile: usize,
    pub population: usize,
    pub roads: HashSet<RoadID>,
    pub area_km2: f64,
    // Relative to the study area, not all of Scotland
    pub density_quintile: usize,
    pub centroid_wgs84: Coord,

    // The bbox, rounded to centimeters, for generate_range to work
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
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
        f.set_property(
            "centroid",
            vec![self.centroid_wgs84.x, self.centroid_wgs84.y],
        );
        f
    }

    pub fn random_point(&self, rng: &mut WyRand) -> Coord {
        loop {
            let x = (rng.generate_range(self.x1..=self.x2) as f64) / 100.0;
            let y = (rng.generate_range(self.y1..=self.y2) as f64) / 100.0;
            let pt = Coord { x, y };
            if self.polygon.contains(&pt) {
                return pt;
            }
        }
    }
}
