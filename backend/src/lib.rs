#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use std::collections::HashMap;

use enum_map::Enum;
use geo::MultiPolygon;
use geojson::{Feature, GeoJson};
use graph::{Graph, RoadID, Timer};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub use crate::existing::Highway;
use crate::level_of_service::LevelOfService;

mod costs;
mod disconnected;
mod evaluate;
pub mod existing;
mod join_lines;
mod level_of_service;
mod mesh_density;
pub mod od;
pub mod places;
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
    settlements: Vec<places::Settlement>,
    data_zones: Vec<places::DataZone>,
    greenspaces: Vec<places::Greenspace>,

    // Per RoadID, static data
    traffic_volumes: Vec<usize>,
    core_network: Vec<Option<Tier>>,
    // Go Dutch totals for all purposes
    precalculated_flows: Vec<usize>,
    // mph
    speeds: Vec<usize>,
    // A percent. Positive if uphill in the forwards direction, negative if downhill
    gradients: Vec<f64>,

    // Stats calculated on a network only with existing infrastructure imported
    baseline_stats: stats::Stats,

    // Derived things per RoadID maintained by recalculate_after_edits
    #[serde(skip_serializing, skip_deserializing, default)]
    infra_types: Vec<Option<InfraType>>,
    /// Does this route have a manually specified InfraType? If so, its LoS is always high
    #[serde(skip_serializing, skip_deserializing, default)]
    override_infra_type: Vec<bool>,
    #[serde(skip_serializing, skip_deserializing, default)]
    tiers: Vec<Option<Tier>>,
    #[serde(skip_serializing, skip_deserializing, default)]
    los: Vec<LevelOfService>,
    #[serde(skip_serializing, skip_deserializing, default)]
    quiet_router_ok: bool,
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
    override_infra_type: bool,
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
        settlements: Vec<places::Settlement>,
        data_zones: Vec<places::DataZone>,
        greenspaces: Vec<places::Greenspace>,
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
        let override_infra_type = std::iter::repeat(false).take(graph.roads.len()).collect();
        let tiers = std::iter::repeat(None).take(graph.roads.len()).collect();
        let los = std::iter::repeat(LevelOfService::ShouldNotBeUsed)
            .take(graph.roads.len())
            .collect();
        let mut model = Self {
            graph,
            routes: HashMap::new(),
            id_counter: 0,
            boundary_wgs84,
            od_zones,
            desire_lines,
            schools,
            gp_hospitals,
            town_centres,
            settlements,
            data_zones,
            greenspaces,
            traffic_volumes,
            core_network,
            precalculated_flows,
            speeds,
            gradients,
            // Calculated below
            baseline_stats: stats::Stats::default(),
            infra_types,
            override_infra_type,
            tiers,
            los,
            quiet_router_ok: false,
        };

        // Calculate baseline stats, relative to existing infrastructure
        let only_some_infra_types = true;
        model.import_existing_routes(only_some_infra_types);
        model.baseline_stats = model.get_stats(&mut Timer::new("calculate baseline stats", None));
        // Clear those edits
        model.clear_all_routes();

        model
    }

    // Just to avoid a pub field for WASM
    pub fn get_baseline_stats_for_cli(&self) -> &stats::Stats {
        &self.baseline_stats
    }

    pub fn recalculate_after_edits(&mut self) {
        self.infra_types = std::iter::repeat(None)
            .take(self.graph.roads.len())
            .collect();
        self.override_infra_type = std::iter::repeat(false)
            .take(self.graph.roads.len())
            .collect();
        self.tiers = std::iter::repeat(None)
            .take(self.graph.roads.len())
            .collect();

        for route in self.routes.values() {
            for (road, _) in &route.roads {
                self.infra_types[road.0] = Some(route.infra_type);
                self.override_infra_type[road.0] = route.override_infra_type;
                self.tiers[road.0] = Some(route.tier);
            }
        }

        self.los = (0..self.graph.roads.len())
            .map(|idx| self.calculate_level_of_service(RoadID(idx)))
            .collect();

        self.quiet_router_ok = false;
    }

    pub fn get_infra_type(&self, r: RoadID) -> InfraType {
        self.infra_types[r.0].unwrap_or(InfraType::MixedTraffic)
    }

    pub fn render_static_roads(&self) -> GeoJson {
        let stats = utils::Quintiles::new(&self.precalculated_flows);
        info!("precalculated_flows quintiles: {stats:?}");

        let mut features = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let mut f = self.graph.mercator.to_wgs84_gj(&road.linestring);
            f.set_property("id", idx);
            f.set_property("way", road.way.to_string());

            f.set_property("traffic", self.traffic_volumes[idx]);
            f.set_property("cn", serde_json::to_value(self.core_network[idx]).unwrap());
            f.set_property("speed", self.speeds[idx]);
            f.set_property("gradient", self.gradients[idx]);
            f.set_property(
                "existing_infra",
                serde_json::to_value(existing::classify(&road.osm_tags)).unwrap(),
            );
            f.set_property("precalculated_flow", self.precalculated_flows[idx]);
            f.set_property(
                "precalculated_flow_quintile",
                stats.quintile(self.precalculated_flows[idx]),
            );

            features.push(f);
        }
        GeoJson::from(features)
    }

    pub fn render_dynamic_roads(&self) -> Vec<DynamicRoad> {
        let reachable = self.get_reachable_network();

        let mut road_to_route = HashMap::new();
        for (route_id, route) in &self.routes {
            for (road, _) in &route.roads {
                road_to_route.insert(*road, *route_id);
            }
        }

        let mut roads = Vec::new();
        for idx in 0..self.graph.roads.len() {
            let id = RoadID(idx);
            let current_route_name = match road_to_route.get(&id) {
                Some(route_id) => {
                    let name = self.routes[route_id].name.clone();
                    Some(if name.is_empty() {
                        format!("Untitled route {route_id}")
                    } else {
                        name
                    })
                }
                None => None,
            };

            roads.push(DynamicRoad {
                id: idx,
                los: self.los[idx],
                reachable: if reachable.network.contains(&id) {
                    "network"
                } else if reachable.severances.contains(&id) {
                    "severance"
                } else if reachable.reachable.contains(&id) {
                    "reachable"
                } else {
                    "unreachable"
                },

                current_route_id: road_to_route.get(&id).cloned(),
                current_route_name,
                current_infra: self.infra_types[idx],
                current_tier: self.tiers[idx],
            });
        }
        roads
    }
}

#[derive(Serialize)]
pub struct DynamicRoad {
    id: usize,
    los: LevelOfService,
    // TODO enum?
    reachable: &'static str,

    // All or nothing
    current_route_id: Option<usize>,
    current_route_name: Option<String>,
    current_infra: Option<InfraType>,
    current_tier: Option<Tier>,
}
