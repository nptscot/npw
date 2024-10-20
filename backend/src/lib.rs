#[macro_use]
extern crate anyhow;

use std::collections::HashMap;

use geojson::Feature;
use graph::{Graph, RoadID, Timer};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod evaluate;
mod existing;
mod mesh_density;
mod od;
mod route_snapper;
mod routes;
mod wasm;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct MapModel {
    graph: Graph,

    #[serde(skip_serializing, skip_deserializing, default)]
    routes: HashMap<usize, Route>,
    #[serde(skip_serializing, skip_deserializing, default)]
    id_counter: usize,
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

#[derive(Clone, Copy, Serialize, Deserialize)]
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
    pub fn create(input_bytes: &[u8], timer: &mut Timer) -> anyhow::Result<MapModel> {
        let modify_roads = |_roads: &mut Vec<graph::Road>| {};
        let graph = Graph::new(
            input_bytes,
            &mut utils::osm2graph::NullReader,
            modify_roads,
            timer,
        )?;

        Ok(MapModel {
            graph,
            routes: HashMap::new(),
            id_counter: 0,
        })
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
