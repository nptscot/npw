#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use std::collections::HashMap;

use enum_map::Enum;
use geo::MultiPolygon;
use geojson::Feature;
use graph::{Graph, RoadID};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod evaluate;
pub mod existing;
mod mesh_density;
pub mod od;
mod route_snapper;
mod routes;
mod stats;
mod wasm;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct MapModel {
    graph: Graph,

    #[serde(skip_serializing, skip_deserializing, default)]
    routes: HashMap<usize, Route>,
    #[serde(skip_serializing, skip_deserializing, default)]
    id_counter: usize,

    // In WGS84
    boundary: MultiPolygon,

    zones: HashMap<String, od::Zone>,
    // TODO Use more compact encoding for zone names
    desire_lines: Vec<(String, String, usize)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Route {
    /// The unedited GeoJSON feature returned from route-snapper
    feature: Feature,
    name: String,
    notes: String,
    roads: Vec<RoadID>,
    infra_type: InfraType,
}

#[derive(Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum InfraType {
    SegregatedWide,
    OffRoad,
    SegregatedNarrow,
    SharedFootway,
    CycleLane,
    MixedTraffic,
    Unknown,
}

impl MapModel {
    // TODO For main.rs to create this. Can't make fields public without wasm_bindgen on them
    pub fn create(
        graph: Graph,
        boundary: MultiPolygon,
        zones: HashMap<String, od::Zone>,
        desire_lines: Vec<(String, String, usize)>,
    ) -> Self {
        Self {
            graph,
            routes: HashMap::new(),
            id_counter: 0,
            boundary,
            zones,
            desire_lines,
        }
    }

    // TODO If this is done frequently, just cache it?
    pub fn get_infra_types(&self) -> HashMap<RoadID, InfraType> {
        let mut infra_types = HashMap::new();
        for route in self.routes.values() {
            for road in &route.roads {
                infra_types.insert(*road, route.infra_type);
            }
        }
        infra_types
    }
}
