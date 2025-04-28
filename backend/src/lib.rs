#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use std::collections::HashMap;

use enum_map::Enum;
use geo::{Area, MultiPolygon, Point};
use geojson::GeoJson;
use graph::{Graph, Intersection, IntersectionID, RoadID, Timer};
use rstar::{primitives::GeomWithData, RTree};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub use crate::existing::Highway;
use crate::level_of_service::LevelOfService;
use crate::routes::{Dir, InMemoryRoute, SavedRoute, SetRouteInput, Waypoint};

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
    closest_intersection_all: RTree<GeomWithData<Point, IntersectionID>>,
    closest_intersection_major: RTree<GeomWithData<Point, IntersectionID>>,

    #[serde(skip_serializing, skip_deserializing, default)]
    routes: HashMap<usize, InMemoryRoute>,
    #[serde(skip_serializing, skip_deserializing, default)]
    id_counter: usize,

    boundary_wgs84: MultiPolygon,

    // (zone1 idx, zone2 idx, count)
    commute_desire_lines: Vec<(usize, usize, usize)>,

    schools: Vec<places::School>,
    gp_hospitals: Vec<places::GPHospital>,
    town_centres: Vec<places::TownCentre>,
    settlements: Vec<places::Settlement>,
    population_zones: Vec<places::PopulationZone>,
    greenspaces: Vec<places::Greenspace>,
    total_settlement_area_m2: f64,

    // Per RoadID, static data
    highways: Vec<Highway>,
    within_settlement: Vec<bool>,
    is_offroad: Vec<bool>,
    traffic_volumes: Vec<usize>,
    core_network: Vec<Option<Tier>>,
    // Go Dutch totals for all purposes
    precalculated_demands: Vec<usize>,
    street_space: Vec<Option<Streetspace>>,
    is_attractive: Vec<bool>,
    // mph
    speeds: Vec<usize>,
    // A percent. Positive if uphill in the forwards direction, negative if downhill
    gradients: Vec<f64>,

    // Stats calculated on a network only with existing infrastructure imported
    baseline_stats: stats::Stats,

    high_demand_threshold: usize,
    medium_demand_threshold: usize,

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

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum, Serialize, Deserialize,
)]
pub enum InfraType {
    Segregated,
    SegregatedWithSpeedVolume,
    OffRoad,
    SharedFootway,
    CycleLane,
    MixedTraffic,
    MixedTrafficWithSpeedVolume,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum, Serialize, Deserialize,
)]
pub enum Tier {
    Primary,
    Secondary,
    LocalAccess,
    LongDistance,
}

/// This is a simplification of the NPT layer
#[derive(Clone, Copy, Debug, Enum, Serialize, Deserialize)]
pub struct Streetspace {
    pub segregated_fits: bool,
}

impl MapModel {
    // TODO For main.rs to create this. Can't make fields public without wasm_bindgen on them
    pub fn create(
        graph: Graph,
        boundary_wgs84: MultiPolygon,
        commute_desire_lines: Vec<(usize, usize, usize)>,
        schools: Vec<places::School>,
        gp_hospitals: Vec<places::GPHospital>,
        town_centres: Vec<places::TownCentre>,
        settlements: Vec<places::Settlement>,
        population_zones: Vec<places::PopulationZone>,
        greenspaces: Vec<places::Greenspace>,
        traffic_volumes: Vec<usize>,
        core_network: Vec<Option<Tier>>,
        street_space: Vec<Option<Streetspace>>,
        is_attractive: Vec<bool>,
        gradients: Vec<f64>,
        timer: &mut Timer,
    ) -> anyhow::Result<Self> {
        timer.step("Finalizing misc fields");
        let highways: Vec<_> = graph
            .roads
            .iter()
            .map(|r| Highway::classify(&r.osm_tags).unwrap())
            .collect();
        let closest_intersection_all = RTree::bulk_load(
            graph
                .intersections
                .iter()
                .map(|i| GeomWithData::new(i.point, i.id))
                .collect(),
        );
        let closest_intersection_major = RTree::bulk_load(
            graph
                .intersections
                .iter()
                .filter(|i| is_major_junction(i, &highways))
                .map(|i| GeomWithData::new(i.point, i.id))
                .collect(),
        );
        let within_settlement = graph
            .roads
            .iter()
            // We could make one set, but this isn't that slow
            .map(|r| settlements.iter().any(|s| s.roads.contains(&r.id)))
            .collect();
        let is_offroad = graph
            .roads
            .iter()
            .enumerate()
            .map(|(idx, r)| is_offroad(highways[idx], &r.osm_tags))
            .collect();
        let total_settlement_area_m2 = settlements.iter().map(|s| s.polygon.unsigned_area()).sum();
        let speeds = graph
            .roads
            .iter()
            .enumerate()
            .map(|(idx, r)| level_of_service::get_speed_mph(highways[idx], &r.osm_tags))
            .collect();
        let infra_types = std::iter::repeat(None).take(graph.roads.len()).collect();
        let override_infra_type = std::iter::repeat(false).take(graph.roads.len()).collect();
        let tiers = std::iter::repeat(None).take(graph.roads.len()).collect();
        let los = std::iter::repeat(LevelOfService::ShouldNotBeUsed)
            .take(graph.roads.len())
            .collect();

        let mut model = Self {
            graph,
            closest_intersection_all,
            closest_intersection_major,
            routes: HashMap::new(),
            id_counter: 0,
            boundary_wgs84,
            commute_desire_lines,
            schools,
            gp_hospitals,
            town_centres,
            settlements,
            population_zones,
            total_settlement_area_m2,
            greenspaces,
            highways,
            within_settlement,
            is_offroad,
            traffic_volumes,
            core_network,
            // Calculated below
            precalculated_demands: Vec::new(),
            street_space,
            is_attractive,
            speeds,
            gradients,
            // Calculated below
            baseline_stats: stats::Stats::default(),
            high_demand_threshold: 0,
            medium_demand_threshold: 0,
            infra_types,
            override_infra_type,
            tiers,
            los,
            quiet_router_ok: false,
        };

        // Calculate baseline stats, relative to existing infrastructure
        let only_some_infra_types = true;
        model.import_existing_routes(only_some_infra_types);
        model.baseline_stats = model.get_stats(timer);
        // Clear those edits
        model.clear_all_routes();

        // Calculate precalculated_demands
        timer.step("precalculate demands");
        model.precalculate_demands()?;

        Ok(model)
    }

    pub fn get_baseline_stats(&self) -> &stats::Stats {
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
        let mut features = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let mut f = self.graph.mercator.to_wgs84_gj(&road.linestring);
            f.set_property("id", idx);
            f.set_property("way", road.way.to_string());
            f.set_property("is_main_road", self.highways[idx].is_main_road());
            f.set_property("within_settlement", self.within_settlement[idx]);
            f.set_property("is_attractive", self.is_attractive[idx]);

            f.set_property("traffic", self.traffic_volumes[idx]);
            f.set_property("cn", serde_json::to_value(self.core_network[idx]).unwrap());
            f.set_property("speed", self.speeds[idx]);
            f.set_property("gradient", self.gradients[idx]);
            f.set_property(
                "existing_infra",
                serde_json::to_value(existing::classify_existing_osm_infra(
                    self.is_offroad[idx],
                    &road.osm_tags,
                ))
                .unwrap(),
            );
            // TODO Could probably just plumb one of these
            f.set_property("precalculated_demand", self.precalculated_demands[idx]);
            f.set_property(
                "precalculated_demand_group",
                if self.precalculated_demands[idx] >= self.high_demand_threshold {
                    "high"
                } else if self.precalculated_demands[idx] >= self.medium_demand_threshold {
                    "medium"
                } else {
                    ""
                },
            );
            f.set_property(
                "street_space",
                serde_json::to_value(self.street_space[idx].map(|ss| {
                    if ss.segregated_fits {
                        "Segregated"
                    } else {
                        "nothing"
                    }
                }))
                .unwrap(),
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
                current_infra_fits: match self.infra_types[idx] {
                    Some(it) => self.does_infra_type_fit(id, it),
                    None => true,
                },
            });
        }
        roads
    }

    pub fn does_infra_type_fit(&self, r: RoadID, infra_type: InfraType) -> bool {
        let Some(streetspace) = self.street_space[r.0] else {
            // Only have this info on main roads. Assume anything can fit on smaller roads. In
            // practice, we won't ask about things like Segregated there anyway.
            return true;
        };
        if matches!(
            infra_type,
            InfraType::Segregated | InfraType::SegregatedWithSpeedVolume
        ) {
            streetspace.segregated_fits
        } else {
            // The other cases fit by definition
            true
        }
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
    current_infra_fits: bool,
}

fn is_offroad(highway: Highway, tags: &::utils::Tags) -> bool {
    if !matches!(
        highway,
        Highway::Footway | Highway::Path | Highway::Cycleway | Highway::Pedestrian
    ) {
        return false;
    }

    // Brittle, but seemingly effective
    if let Some(name) = tags.get("name") {
        return [
            " Path", " Towpath", " Railway", " Trail", " Walkway", " Walk",
        ]
        .iter()
        .any(|x| name.ends_with(x));
    }

    false
}

pub fn is_major_junction(intersection: &Intersection, highways: &Vec<Highway>) -> bool {
    intersection
        .roads
        .iter()
        .filter(|r| highways[r.0].is_main_road())
        .count()
        >= 3
}
