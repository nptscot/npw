#[macro_use]
extern crate anyhow;

use std::sync::Once;

use graph::{Graph, Timer};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

mod route_snapper;

static START: Once = Once::new();

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct MapModel {
    graph: Graph,
}

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
}

// TODO Move non-WASM methods?
impl MapModel {
    pub fn create(input_bytes: &[u8], timer: &mut Timer) -> anyhow::Result<MapModel> {
        let modify_roads = |_roads: &mut Vec<graph::Road>| {};
        let graph = Graph::new(
            input_bytes,
            &mut utils::osm2graph::NullReader,
            modify_roads,
            timer,
        )?;

        Ok(MapModel { graph })
    }
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
