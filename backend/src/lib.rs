#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use std::collections::{HashMap, HashSet};
use std::time::Duration;

use chrono::NaiveTime;
use enum_map::Enum;
use geo::MultiPolygon;
use geojson::Feature;
use graph::{Graph, RoadID};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod evaluate;
pub mod existing;
mod level_of_service;
mod mesh_density;
pub mod od;
pub mod places;
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

    boundary_wgs84: MultiPolygon,

    zones: HashMap<String, od::Zone>,
    // TODO Use more compact encoding for zone names
    desire_lines: Vec<(String, String, usize)>,

    schools: Vec<places::School>,

    // Per RoadID
    traffic_volumes: Vec<usize>,
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

#[derive(Clone, Copy, Debug, PartialEq, Enum, Serialize, Deserialize)]
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
        boundary_wgs84: MultiPolygon,
        zones: HashMap<String, od::Zone>,
        desire_lines: Vec<(String, String, usize)>,
        schools: Vec<places::School>,
        traffic_volumes: Vec<usize>,
    ) -> Self {
        Self {
            graph,
            routes: HashMap::new(),
            id_counter: 0,
            boundary_wgs84,
            zones,
            desire_lines,
            schools,
            traffic_volumes,
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

    /// All roads within some predefined buffer of the defined (and maybe existing) network
    pub fn get_network_buffer(&self, include_existing: bool) -> Vec<RoadID> {
        let mut starts = HashSet::new();
        for route in self.routes.values() {
            for r in &route.roads {
                let road = &self.graph.roads[r.0];
                starts.insert(road.src_i);
                starts.insert(road.dst_i);
            }
        }

        if include_existing {
            for road in &self.graph.roads {
                if existing::classify(&road.osm_tags).is_some() {
                    starts.insert(road.src_i);
                    starts.insert(road.dst_i);
                }
            }
        }

        let start_time = NaiveTime::from_hms_opt(7, 0, 0).unwrap();
        // TODO Distance buffer?
        let limit = Duration::from_secs(30);
        let public_transit = false;
        let cost_per_road = self.graph.get_costs(
            starts.into_iter().collect(),
            self.graph.profile_names["bicycle"],
            public_transit,
            start_time,
            start_time + limit,
        );

        cost_per_road.into_keys().collect()
    }
}
