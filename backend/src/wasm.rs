use std::collections::HashMap;
use std::sync::Once;

use geo::{Coord, LineString, Polygon};
use geojson::{Feature, FeatureCollection, Geometry};
use graph::{IntersectionID, RoadID, Timer};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{evaluate::Breakdown, Dir, Highway, InfraType, MapModel, Route, Tier};

static START: Once = Once::new();

#[wasm_bindgen]
impl MapModel {
    #[wasm_bindgen(constructor)]
    pub fn new(input_bytes: &[u8]) -> Result<MapModel, JsValue> {
        // Panics shouldn't happen, but if they do, console.log them.
        console_error_panic_hook::set_once();
        START.call_once(|| {
            console_log::init_with_level(log::Level::Info).unwrap();
        });

        info!("Deserializing MapModel from {} bytes", input_bytes.len());
        let mut map: MapModel = bincode::deserialize_from(input_bytes).map_err(err_to_js)?;
        map.recalculate_after_edits();
        Ok(map)
    }

    /// Returns a GeoJSON string. Just shows the full network
    #[wasm_bindgen(js_name = renderDebug)]
    pub fn render_debug(&self) -> Result<String, JsValue> {
        let fc = self.graph.render_debug();
        serde_json::to_string(&fc).map_err(err_to_js)
    }

    /// Return a polygon covering the world, minus a hole for the boundary, in WGS84
    #[wasm_bindgen(js_name = getInvertedBoundary)]
    pub fn get_inverted_boundary(&self) -> Result<String, JsValue> {
        // TODO Assume none of the boundary polygons have holes
        let polygon = Polygon::new(
            LineString::from(vec![
                (180.0, 90.0),
                (-180.0, 90.0),
                (-180.0, -90.0),
                (180.0, -90.0),
                (180.0, 90.0),
            ]),
            self.boundary_wgs84
                .0
                .iter()
                .map(|p| p.exterior().clone())
                .collect(),
        );
        let f = Feature::from(Geometry::from(&polygon));
        let out = serde_json::to_string(&f).map_err(err_to_js)?;
        Ok(out)
    }

    /// WGS84
    #[wasm_bindgen(js_name = getBounds)]
    pub fn get_bounds(&self) -> Vec<f64> {
        let b = &self.graph.mercator.wgs84_bounds;
        vec![b.min().x, b.min().y, b.max().x, b.max().y]
    }

    #[wasm_bindgen(js_name = toRouteSnapper)]
    pub fn to_route_snapper(&self) -> Vec<u8> {
        bincode::serialize(&self.to_route_snapper_graph()).unwrap()
    }

    /// Create or edit a route. Returns the ID
    #[wasm_bindgen(js_name = setRoute)]
    pub fn set_route_wasm(&mut self, id: Option<usize>, input: JsValue) -> Result<(), JsValue> {
        let route = self.parse_route(input).map_err(err_to_js)?;
        self.set_route(id, route).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = deleteRoute)]
    pub fn delete_route_wasm(&mut self, id: usize) -> Result<(), JsValue> {
        self.delete_route(id).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = clearAllRoutes)]
    pub fn clear_all_routes_wasm(&mut self) {
        self.clear_all_routes()
    }

    /// Splits a route into sections, returning a FeatureCollection
    #[wasm_bindgen(js_name = autosplitRoute)]
    pub fn autosplit_route_wasm(&self, input: JsValue) -> Result<String, JsValue> {
        // TODO Or take a full Route as input and reuse parse_route?
        let full_path: Vec<RouteNode> = serde_wasm_bindgen::from_value(input)?;
        let roads = self.full_path_to_roads(full_path).map_err(err_to_js)?;
        self.autosplit_route(roads).map_err(err_to_js)
    }

    /// Returns a GeoJSON string showing all routes
    #[wasm_bindgen(js_name = renderRoutes)]
    pub fn render_routes(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.to_routes_gj()).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = evaluateRoute)]
    pub fn evaluate_route_wasm(&self, input: JsValue) -> Result<String, JsValue> {
        let req: EvaluateRouteRequest = serde_wasm_bindgen::from_value(input)?;
        self.evaluate_route(
            self.graph.mercator.pt_to_mercator(Coord {
                x: req.x1,
                y: req.y1,
            }),
            self.graph.mercator.pt_to_mercator(Coord {
                x: req.x2,
                y: req.y2,
            }),
            match req.breakdown.as_str() {
                "" => Breakdown::None,
                "los" => Breakdown::LevelOfService,
                "infra_type" => Breakdown::InfraType,
                "gradient" => Breakdown::Gradient,
                x => {
                    return Err(err_to_js(format!("evaluateRoute got bad breakdown {x}")));
                }
            },
        )
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = debugReachablePath)]
    pub fn debug_reachable_path_wasm(&self, kind: &str, idx: usize) -> Result<String, JsValue> {
        let roads = match kind {
            "schools" => [self.schools[idx].road].into(),
            "gp_hospitals" => [self.gp_hospitals[idx].road].into(),
            "town_centres" => self.town_centres[idx].roads.clone(),
            "settlements" => self.settlements[idx].roads.clone(),
            _ => {
                return Err(err_to_js(format!(
                    "debug_reachable_path_wasm got bad kind {kind}"
                )));
            }
        };
        self.debug_reachable_path(roads).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = debugUnreachablePath)]
    pub fn debug_unrreachable_path_wasm(&self, kind: &str, idx: usize) -> Result<String, JsValue> {
        let roads = match kind {
            "schools" => [self.schools[idx].road].into(),
            "gp_hospitals" => [self.gp_hospitals[idx].road].into(),
            "town_centres" => self.town_centres[idx].roads.clone(),
            "settlements" => self.settlements[idx].roads.clone(),
            _ => {
                return Err(err_to_js(format!(
                    "debug_reachable_path_wasm got bad kind {kind}"
                )));
            }
        };
        self.debug_unreachable_path(roads).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = evaluateOD)]
    pub fn evaluate_od_wasm(&self) -> Result<String, JsValue> {
        self.evaluate_od().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = recalculateStats)]
    pub fn recalculate_stats_wasm(&mut self) -> Result<String, JsValue> {
        let mut timer = Timer::new("recalculate after edits", None);
        let result = self.recalculate_stats(&mut timer).map_err(err_to_js);
        timer.done();
        result
    }

    #[wasm_bindgen(js_name = toSavefile)]
    pub fn to_savefile(&self) -> Result<String, JsValue> {
        serde_json::to_string(&Savefile {
            routes: self.routes.clone(),
            id_counter: self.id_counter,
        })
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = loadSavefile)]
    pub fn load_savefile(&mut self, input: String) -> Result<(), JsValue> {
        let savefile: Savefile = serde_json::from_str(&input).map_err(err_to_js)?;
        // TODO Detect if the savefile is incompatible with the current model (too big RoadIDs) and
        // bail cleanly
        self.routes = savefile.routes;
        self.id_counter = savefile.id_counter;
        self.recalculate_after_edits();
        Ok(())
    }

    #[wasm_bindgen(js_name = meshDensity)]
    pub fn mesh_density(&self) -> Result<String, JsValue> {
        self.calculate_mesh_density().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = classifyExistingNetwork)]
    pub fn classify_existing_network_wasm(&self) -> Result<String, JsValue> {
        self.classify_existing_network().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = importExistingRoutes)]
    pub fn import_existing_routes_wasm(&mut self) -> usize {
        self.import_existing_routes()
    }

    #[wasm_bindgen(js_name = importCoreNetwork)]
    pub fn import_core_network_wasm(&mut self) -> usize {
        self.import_core_network()
    }

    #[wasm_bindgen(js_name = getSchools)]
    pub fn get_schools(&self) -> Result<String, JsValue> {
        // TODO Some kind of caching would make this nicer
        let roads = self.get_reachable_network();

        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .schools
                .iter()
                .enumerate()
                .map(|(idx, s)| s.to_gj(&self.graph.mercator, roads.covers(s.road), idx))
                .collect(),
        })
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getGPHospitals)]
    pub fn get_gp_hospitals(&self) -> Result<String, JsValue> {
        // TODO Some kind of caching would make this nicer
        let roads = self.get_reachable_network();

        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .gp_hospitals
                .iter()
                .enumerate()
                .map(|(idx, x)| x.to_gj(&self.graph.mercator, roads.covers(x.road), idx))
                .collect(),
        })
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getTownCentres)]
    pub fn get_town_centres(&self) -> Result<String, JsValue> {
        // TODO Some kind of caching would make this nicer
        let roads = self.get_reachable_network();

        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .town_centres
                .iter()
                .enumerate()
                .map(|(idx, x)| x.to_gj(&self.graph.mercator, roads.covers_any(&x.roads), idx))
                .collect(),
        })
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getSettlements)]
    pub fn get_settlements(&self) -> Result<String, JsValue> {
        // TODO Some kind of caching would make this nicer
        let roads = self.get_reachable_network();

        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .settlements
                .iter()
                .enumerate()
                .map(|(idx, x)| x.to_gj(&self.graph.mercator, roads.covers_any(&x.roads), idx))
                .collect(),
        })
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getDataZones)]
    pub fn get_data_zones(&self) -> Result<String, JsValue> {
        // TODO Some kind of caching would make this nicer
        let roads = self.get_reachable_network();

        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .data_zones
                .iter()
                .map(|x| x.to_gj(&self.graph.mercator, roads.covers_any(&x.roads)))
                .collect(),
        })
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = renderReachableNetwork)]
    pub fn render_reachable_network_wasm(&self) -> Result<String, JsValue> {
        self.render_reachable_network().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = renderLevelOfService)]
    pub fn render_level_of_service_wasm(&self) -> Result<String, JsValue> {
        self.render_level_of_service().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = renderCoreNetwork)]
    pub fn render_core_network(&self) -> Result<String, JsValue> {
        let mut features = Vec::new();
        for (road, tier) in self.graph.roads.iter().zip(self.core_network.iter()) {
            if let Some(tier) = tier {
                let mut f = self.graph.mercator.to_wgs84_gj(&road.linestring);
                f.set_property("tier", serde_json::to_value(tier).map_err(err_to_js)?);
                features.push(f);
            }
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })
        .map_err(err_to_js)?)
    }

    #[wasm_bindgen(js_name = renderPrecalculatedFlows)]
    pub fn render_precalculated_flows_wasm(&self) -> Result<String, JsValue> {
        self.render_precalculated_flows().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getMajorJunctions)]
    pub fn get_major_junctions(&self) -> Result<String, JsValue> {
        let mut features = Vec::new();
        for i in &self.graph.intersections {
            if i.roads
                .iter()
                .filter(|r| {
                    matches!(
                        Highway::classify(&self.graph.roads[r.0].osm_tags).unwrap(),
                        Highway::Motorway
                            | Highway::Trunk
                            | Highway::Primary
                            | Highway::Secondary
                            | Highway::Tertiary
                    )
                })
                .count()
                >= 3
            {
                features.push(self.graph.mercator.to_wgs84_gj(&i.point));
            }
        }

        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features,
        })
        .map_err(err_to_js)
    }

    fn parse_route(&self, input: JsValue) -> anyhow::Result<Route> {
        // TODO map_err?
        let route: InputRoute = match serde_wasm_bindgen::from_value(input) {
            Ok(r) => r,
            Err(err) => bail!("{err}"),
        };
        Ok(Route {
            feature: route.feature,
            name: route.name,
            notes: route.notes,
            roads: self.full_path_to_roads(route.full_path)?,
            infra_type: route.infra_type,
            tier: route.tier,
        })
    }

    // Returns (Road, forwards) pairs
    fn full_path_to_roads(&self, full_path: Vec<RouteNode>) -> anyhow::Result<Vec<(RoadID, Dir)>> {
        let mut intersections = Vec::new();
        for node in full_path {
            if let Some(id) = node.snapped {
                intersections.push(IntersectionID(id as usize));
            } else if node.free.is_some() {
                bail!("can't handle freehand points yet");
            } else {
                bail!("input has a blank node");
            }
        }

        let mut roads = Vec::new();
        for pair in intersections.windows(2) {
            match self.graph.find_edge(pair[0], pair[1]) {
                Some(road) => {
                    roads.push((
                        road.id,
                        if road.src_i == pair[0] {
                            Dir::Forwards
                        } else {
                            Dir::Backwards
                        },
                    ));
                }
                None => {
                    // TODO Change route snapper behavior here? Or treat as a freehand line?
                    bail!("no path between some waypoints");
                }
            }
        }
        Ok(roads)
    }
}

#[derive(Deserialize)]
struct InputRoute {
    feature: Feature,
    name: String,
    notes: String,
    full_path: Vec<RouteNode>,
    infra_type: InfraType,
    tier: Tier,
}

#[derive(Deserialize)]
struct RouteNode {
    snapped: Option<u32>,
    free: Option<[f64; 2]>,
}

#[derive(Deserialize)]
struct EvaluateRouteRequest {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    breakdown: String,
}

// TODO This is an odd, repetitive format. Redesign later.
#[derive(Serialize, Deserialize)]
struct Savefile {
    routes: HashMap<usize, Route>,
    id_counter: usize,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
