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

use crate::{existing::Highway, level_of_service::LevelOfService};

mod costs;
mod evaluate;
pub mod existing;
mod join_lines;
mod level_of_service;
mod mesh_density;
pub mod od;
pub mod places;
mod precalculated_flow;
mod reachable;
mod route_snapper;
mod routes;
mod stats;
mod uptake;
mod utils;
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

    od_zones: HashMap<String, od::Zone>,
    // TODO Use more compact encoding for zone names
    desire_lines: Vec<(String, String, usize)>,

    schools: Vec<places::School>,
    gp_hospitals: Vec<places::GPHospital>,
    town_centres: Vec<places::TownCentre>,
    data_zones: Vec<places::DataZone>,

    // Per RoadID
    traffic_volumes: Vec<usize>,
    core_network: Vec<Option<Tier>>,
    // Go Dutch totals for all purposes
    precalculated_flows: Vec<usize>,
    // mph
    speeds: Vec<usize>,
    // A percent. Positive if uphill in the forwards direction, negative if downhill
    gradients: Vec<f64>,
    // Derived things maintained by recalculate_after_edits
    #[serde(skip_serializing, skip_deserializing, default)]
    infra_types: Vec<Option<InfraType>>,
    #[serde(skip_serializing, skip_deserializing, default)]
    los: Vec<LevelOfService>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Route {
    /// The unedited GeoJSON feature returned from route-snapper
    feature: Feature,
    name: String,
    notes: String,
    // Derived from full_path. The direction is only plumbed along for rendering/splitting purposes
    roads: Vec<(RoadID, Dir)>,
    infra_type: InfraType,
    tier: Tier,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Dir {
    Forwards,
    Backwards,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum, Serialize, Deserialize,
)]
pub enum InfraType {
    SegregatedWide,
    OffRoad,
    SegregatedNarrow,
    SharedFootway,
    CycleLane,
    MixedTraffic,
    Unknown,
}

#[derive(Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub enum Tier {
    Primary,
    Secondary,
    LocalAccess,
    LongDistance,
}

impl MapModel {
    // TODO For main.rs to create this. Can't make fields public without wasm_bindgen on them
    pub fn create(
        graph: Graph,
        boundary_wgs84: MultiPolygon,
        od_zones: HashMap<String, od::Zone>,
        desire_lines: Vec<(String, String, usize)>,
        schools: Vec<places::School>,
        gp_hospitals: Vec<places::GPHospital>,
        town_centres: Vec<places::TownCentre>,
        data_zones: Vec<places::DataZone>,
        traffic_volumes: Vec<usize>,
        core_network: Vec<Option<Tier>>,
        precalculated_flows: Vec<usize>,
        gradients: Vec<f64>,
    ) -> Self {
        let speeds = graph
            .roads
            .iter()
            .map(|r| level_of_service::get_speed_mph(&r.osm_tags))
            .collect();
        let infra_types = std::iter::repeat(None).take(graph.roads.len()).collect();
        let los = std::iter::repeat(LevelOfService::ShouldNotBeUsed)
            .take(graph.roads.len())
            .collect();
        Self {
            graph,
            routes: HashMap::new(),
            id_counter: 0,
            boundary_wgs84,
            od_zones,
            desire_lines,
            schools,
            gp_hospitals,
            town_centres,
            data_zones,
            traffic_volumes,
            core_network,
            precalculated_flows,
            speeds,
            gradients,
            infra_types,
            los,
        }
    }

    pub fn recalculate_after_edits(&mut self) {
        self.infra_types = std::iter::repeat(None)
            .take(self.graph.roads.len())
            .collect();

        for route in self.routes.values() {
            for (road, _) in &route.roads {
                self.infra_types[road.0] = Some(route.infra_type);
            }
        }

        self.los = (0..self.graph.roads.len())
            .map(|idx| self.calculate_level_of_service(RoadID(idx)))
            .collect();
    }

    pub fn get_infra_type(&self, r: RoadID) -> InfraType {
        self.infra_types[r.0].unwrap_or(InfraType::MixedTraffic)
    }
}
