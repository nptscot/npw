use std::collections::HashSet;

use geo::{Contains, Coord, MultiPolygon, Point, Polygon};
use geojson::Feature;
use graph::RoadID;
use nanorand::{Rng, WyRand};
use serde::{Deserialize, Serialize};
use utils::Mercator;

// TODO We can't use geojson::ser::to_feature_collection_string and similar magic, because bincode
// doesn't work with it, and we need to do the CRS transform

#[derive(Serialize, Deserialize)]
pub struct School {
    pub point: Point,
    pub kind: String,
    pub name: String,
    // TODO Fix upstream
    pub pupils: usize,
    pub road: RoadID,
    pub sort: f64,
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
}

#[derive(Serialize, Deserialize)]
pub struct GPHospital {
    pub point: Point,
    pub kind: String,
    pub name: String,
    pub road: RoadID,
    pub sort: f64,
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
}

#[derive(Serialize, Deserialize)]
pub struct RailwayStation {
    pub point: Point,
    pub name: Option<String>,
    pub road: RoadID,
    pub sort: f64,
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
}

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

#[derive(Serialize, Deserialize)]
pub struct TownCentre {
    pub polygon: Polygon,
    pub name: Option<String>,
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
}

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
}

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
