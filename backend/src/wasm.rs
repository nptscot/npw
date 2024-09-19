use std::sync::Once;

use geojson::Feature;
use graph::IntersectionID;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

use crate::{InfraType, MapModel, Route};

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

        bincode::deserialize_from(input_bytes).map_err(err_to_js)
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
        self.graph.get_inverted_boundary().map_err(err_to_js)
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

    /// Returns the ID of the new route
    #[wasm_bindgen(js_name = newRoute)]
    pub fn new_route(&mut self, input: JsValue) -> Result<usize, JsValue> {
        let route = self.parse_route(input).map_err(err_to_js)?;
        self.add_route(route).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = deleteRoute)]
    pub fn delete_route_wasm(&mut self, id: usize) -> Result<(), JsValue> {
        self.delete_route(id).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = editRoute)]
    pub fn edit_route_wasm(&mut self, id: usize, input: JsValue) -> Result<(), JsValue> {
        let route = self.parse_route(input).map_err(err_to_js)?;
        self.edit_route(id, route).map_err(err_to_js)
    }

    /// Returns a GeoJSON string showing all routes
    #[wasm_bindgen(js_name = renderRoutes)]
    pub fn render_routes(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.to_routes_gj()).map_err(err_to_js)
    }

    fn parse_route(&self, input: JsValue) -> anyhow::Result<Route> {
        // TODO map_err?
        let route: InputRoute = match serde_wasm_bindgen::from_value(input) {
            Ok(r) => r,
            Err(err) => bail!("{err}"),
        };

        let mut intersections = Vec::new();
        for node in route.nodes {
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
                    roads.push(road.id);
                }
                None => {
                    // TODO Change route snapper behavior here? Or treat as a freehand line?
                    bail!("no path between some waypoints");
                }
            }
        }

        Ok(Route {
            feature: route.feature,
            name: route.name,
            notes: route.notes,
            roads,
            infra_type: route.infra_type,
        })
    }
}

#[derive(Deserialize)]
struct InputRoute {
    feature: Feature,
    name: String,
    notes: String,
    nodes: Vec<RouteNode>,
    infra_type: InfraType,
}

#[derive(Deserialize)]
struct RouteNode {
    snapped: Option<u32>,
    free: Option<[f64; 2]>,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
