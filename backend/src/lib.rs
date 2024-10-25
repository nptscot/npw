#[macro_use]
extern crate anyhow;

use std::collections::HashMap;

use anyhow::Result;
use enum_map::Enum;
use geo::MultiPolygon;
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
    pub fn create(input_bytes: &[u8], boundary_gj: &str, timer: &mut Timer) -> Result<MapModel> {
        let graph = Graph::new(
            input_bytes,
            &mut utils::osm2graph::NullReader,
            vec![("bicycle".to_string(), Box::new(existing::bicycle_profile))],
            timer,
        )?;

        Ok(MapModel {
            graph,
            routes: HashMap::new(),
            id_counter: 0,
            boundary: read_multipolygon(boundary_gj)?,
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

fn read_multipolygon(gj_string: &str) -> Result<MultiPolygon> {
    let gj: geojson::Feature = gj_string.parse()?;
    if matches!(
        gj.geometry.as_ref().unwrap().value,
        geojson::Value::Polygon(_)
    ) {
        Ok(MultiPolygon(vec![gj.try_into()?]))
    } else {
        Ok(gj.try_into()?)
    }
}
