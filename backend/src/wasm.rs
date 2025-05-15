use std::collections::HashSet;
use std::sync::Once;

use geo::{BoundingRect, Centroid, Coord, LineString, MultiPolygon, Polygon, Rect};
use geojson::{Feature, FeatureCollection, Geometry};
use graph::{RoadID, Timer};
use serde::Deserialize;
use serde_json::Value;
use wasm_bindgen::prelude::*;

use crate::{evaluate::Breakdown, InfraType, MapModel, SavedRoute, SetRouteInput, Tier, Waypoint};

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

    /// Returns GJ with one feature per road, with all properties that never change.
    #[wasm_bindgen(js_name = renderStaticRoads)]
    pub fn render_static_roads_wasm(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.render_static_roads()).map_err(err_to_js)
    }

    /// Returns a list of objects per road, with all properties that do change.
    #[wasm_bindgen(js_name = renderDynamicRoads)]
    pub fn render_dynamic_roads_wasm(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.render_dynamic_roads()).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getInvertedBoundaryInsideSettlements)]
    pub fn get_inverted_boundary_inside_settlements(&self) -> Result<String, JsValue> {
        let the_world = LineString::from(vec![
            (180.0, 90.0),
            (-180.0, 90.0),
            (-180.0, -90.0),
            (180.0, -90.0),
            (180.0, 90.0),
        ]);
        let mut holes = Vec::new();
        for s in &self.settlements {
            holes.push(self.graph.mercator.to_wgs84(&s.polygon).exterior().clone());
        }
        let polygon = Polygon::new(the_world, holes);
        let f = Feature::from(Geometry::from(&polygon));
        let out = serde_json::to_string(&f).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = getInvertedBoundaryOutsideSettlements)]
    pub fn get_inverted_boundary_outside_settlements(&self) -> Result<String, JsValue> {
        let the_world = LineString::from(vec![
            (180.0, 90.0),
            (-180.0, 90.0),
            (-180.0, -90.0),
            (180.0, -90.0),
            (180.0, 90.0),
        ]);
        let mut polygons = Vec::new();
        // Fade out everything except the study area
        polygons.push(Polygon::new(
            the_world,
            self.boundary_wgs84
                .0
                .iter()
                .map(|p| p.exterior().clone())
                .collect(),
        ));

        // Also fade out all of the settlements, leaving just the rural space in between
        for s in &self.settlements {
            polygons.push(self.graph.mercator.to_wgs84(&s.polygon));
        }
        let f = Feature::from(Geometry::from(&MultiPolygon(polygons)));
        let out = serde_json::to_string(&f).map_err(err_to_js)?;
        Ok(out)
    }

    #[wasm_bindgen(js_name = getInvertedBoundaryForStudyArea)]
    pub fn get_inverted_boundary_for_study_area(&self) -> Result<String, JsValue> {
        let the_world = LineString::from(vec![
            (180.0, 90.0),
            (-180.0, 90.0),
            (-180.0, -90.0),
            (180.0, -90.0),
            (180.0, 90.0),
        ]);
        // Fade out everything except the study area
        let polygon = Polygon::new(
            the_world,
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

    #[wasm_bindgen(js_name = getStudyAreaBoundary)]
    pub fn get_study_area_boundary(&self) -> Result<String, JsValue> {
        let f = Feature::from(Geometry::from(&self.boundary_wgs84));
        let out = serde_json::to_string(&f).map_err(err_to_js)?;
        Ok(out)
    }

    /// WGS84
    #[wasm_bindgen(js_name = getBounds)]
    pub fn get_bounds(&self) -> Vec<f64> {
        let b = &self.graph.mercator.wgs84_bounds;
        vec![b.min().x, b.min().y, b.max().x, b.max().y]
    }

    /// Create or edit a route. Returns the list of route IDs
    #[wasm_bindgen(js_name = setRoute)]
    pub fn set_route_wasm(
        &mut self,
        id: Option<usize>,
        input: JsValue,
    ) -> Result<Vec<usize>, JsValue> {
        let mut route: SetRouteInput = serde_wasm_bindgen::from_value(input)?;
        for w in &mut route.waypoints {
            self.to_mercator(&mut w.point);
        }

        self.set_route(id, route).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = deleteRoutes)]
    pub fn delete_routes_wasm(&mut self, ids: Vec<usize>) -> Result<(), JsValue> {
        self.delete_routes(ids).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = changeTier)]
    pub fn change_tier_wasm(&mut self, route_ids: Vec<usize>, tier: String) -> Result<(), JsValue> {
        let tier: Tier = serde_json::from_str(&tier).map_err(err_to_js)?;
        self.change_tier(route_ids, tier).map_err(err_to_js)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = changeInfraType)]
    pub fn change_infra_type_wasm(
        &mut self,
        route_ids: Vec<usize>,
        infra_type: String,
    ) -> Result<(), JsValue> {
        let infra_type: InfraType = serde_json::from_str(&infra_type).map_err(err_to_js)?;
        self.change_infra_type(route_ids, infra_type)
            .map_err(err_to_js)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = clearAllRoutes)]
    pub fn clear_all_routes_wasm(&mut self) {
        self.clear_all_routes()
    }

    /// Splits a route into sections, returning a FeatureCollection
    #[wasm_bindgen(js_name = autosplitRoute)]
    pub fn autosplit_route_wasm(
        &self,
        editing_route_id: Option<usize>,
        raw_waypoints: JsValue,
        override_infra_type: JsValue,
        default_tier: String,
        major_snap_threshold: Option<f64>,
    ) -> Result<String, JsValue> {
        let mut waypoints: Vec<Waypoint> = serde_wasm_bindgen::from_value(raw_waypoints)?;
        for w in &mut waypoints {
            self.to_mercator(&mut w.point);
        }

        let override_infra_type: Option<InfraType> =
            serde_wasm_bindgen::from_value(override_infra_type)?;
        let default_tier: Tier = serde_json::from_str(&default_tier).map_err(err_to_js)?;
        self.autosplit_route(
            editing_route_id,
            waypoints,
            override_infra_type,
            default_tier,
            major_snap_threshold,
        )
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = snapPoint)]
    pub fn snap_point(&self, lon: f64, lat: f64, major_snap_threshold: Option<f64>) -> Vec<f64> {
        let pt = self.graph.mercator.pt_to_mercator(Coord { x: lon, y: lat });
        let i = self.snap_to_intersection(pt.into(), major_snap_threshold);
        let snapped = self
            .graph
            .mercator
            .pt_to_wgs84(self.graph.intersections[i.0].point.into());
        vec![snapped.x, snapped.y]
    }

    /// Returns GJ Features of every route
    #[wasm_bindgen(js_name = getAllRoutes)]
    pub fn get_all_routes_wasm(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.get_all_routes()).map_err(err_to_js)
    }

    /// Returns one GJ Feature of the route
    #[wasm_bindgen(js_name = getRoute)]
    pub fn get_route_wasm(&self, id: usize) -> Result<String, JsValue> {
        let route = self.get_route(id).map_err(err_to_js)?;
        serde_json::to_string(&route).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getRouteSections)]
    pub fn get_route_sections_wasm(&self, ids: Vec<usize>) -> Result<String, JsValue> {
        self.get_route_sections(ids).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = evaluateRoute)]
    pub fn evaluate_route_wasm(&mut self, input: JsValue) -> Result<String, JsValue> {
        if !self.quiet_router_ok {
            let mut timer = Timer::new("recalculate bicycle_quiet", None);
            self.recalculate_quiet_router(&mut timer);
        }

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
        let roads = self.get_poi_roads(kind, idx)?;
        self.debug_reachable_path(roads).map_err(err_to_js)
    }

    // TODO Unused
    #[wasm_bindgen(js_name = debugUnreachablePath)]
    pub fn debug_unreachable_path_wasm(&self, kind: &str, idx: usize) -> Result<String, JsValue> {
        let roads = self.get_poi_roads(kind, idx)?;
        self.debug_unreachable_path(roads).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = fixUnreachablePOI)]
    pub fn fix_unreachable_poi_wasm(&self, kind: &str, idx: usize) -> Result<String, JsValue> {
        let roads = self.get_poi_roads(kind, idx)?;
        self.fix_unreachable_poi(roads).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = evaluateOD)]
    pub fn evaluate_od_wasm(&mut self, fast_sample: bool) -> Result<String, JsValue> {
        if !self.quiet_router_ok {
            let mut timer = Timer::new("recalculate bicycle_quiet", None);
            self.recalculate_quiet_router(&mut timer);
        }

        self.evaluate_od(fast_sample).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = recalculateStats)]
    pub fn recalculate_stats_wasm(&mut self) -> Result<String, JsValue> {
        let mut timer = Timer::new("recalculate fast stats", None);
        let result = serde_json::to_string(&self.get_stats(&mut timer)).map_err(err_to_js);
        timer.done();
        result
    }

    #[wasm_bindgen(js_name = recalculateSlowStats)]
    pub fn recalculate_slow_stats_wasm(&mut self) -> Result<String, JsValue> {
        let mut timer = Timer::new("recalculate slow stats", None);
        if !self.quiet_router_ok {
            self.recalculate_quiet_router(&mut timer);
        }

        let result = serde_json::to_string(&self.get_slow_stats(&mut timer)).map_err(err_to_js);
        timer.done();
        result
    }

    /// Includes slow stats too
    #[wasm_bindgen(js_name = getBaselineStats)]
    pub fn get_baseline_stats_wasm(&self) -> Result<String, JsValue> {
        let mut props = crate::utils::into_object_value(
            serde_json::to_value(&self.baseline_stats).map_err(err_to_js)?,
        );
        props.insert(
            "average_weighted_directness".to_string(),
            self.baseline_slow_stats.average_weighted_directness.into(),
        );
        serde_json::to_string(&props).map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = recalculateODStats)]
    pub fn recalculate_od_stats_wasm(&mut self) -> Result<String, JsValue> {
        let mut timer = Timer::new("recalculate OD stats", None);
        let result = self.recalculate_od_stats(&mut timer).map_err(err_to_js);
        timer.done();
        result
    }

    #[wasm_bindgen(js_name = loadSavefile)]
    pub fn load_savefile(&mut self, input: String) -> Result<(), JsValue> {
        let savefile: FeatureCollection = serde_json::from_str(&input).map_err(err_to_js)?;
        let Some(foreign_members) = savefile.foreign_members.as_ref() else {
            return Err(JsValue::from_str(
                "GeoJSON is missing foreign members section",
            ));
        };

        let mut ok = false;
        if let Some(Value::Number(num)) = foreign_members.get("version") {
            if let Some(version) = num.as_u64() {
                ok = version == 2;
            }
        }
        if !ok {
            return Err(JsValue::from_str("Savefile is out-of-date"));
        }

        let Some(Value::String(study_area_name)) = foreign_members.get("study_area_name") else {
            return Err(JsValue::from_str("Savefile is mising study_area_name"));
        };
        if study_area_name != &self.study_area_name {
            return Err(JsValue::from_str(&format!(
                "Savefile is for {study_area_name}, but you are currently in {}",
                self.study_area_name
            )));
        }

        self.id_counter = match foreign_members.get("id_counter") {
            Some(Value::Number(num)) => match num.as_u64() {
                Some(num) => num as usize,
                None => {
                    return Err(JsValue::from_str("Savefile has bad id_counter"));
                }
            },
            _ => {
                return Err(JsValue::from_str("Savefile is missing id_counter"));
            }
        };

        for feature in savefile.features {
            let route: SavedRoute = geojson::de::from_feature(feature).map_err(err_to_js)?;
            self.routes.insert(route.id, route.to_in_memory(self));
        }

        self.recalculate_after_edits();
        Ok(())
    }

    #[wasm_bindgen(js_name = getGridMeshDensity)]
    pub fn get_grid_mesh_density(
        &self,
        resolution: f64,
        x_offset: f64,
        y_offset: f64,
    ) -> Result<String, JsValue> {
        self.calculate_grid_mesh_density(resolution, x_offset, y_offset)
            .map_err(err_to_js)
    }

    /// true means only import some infra types, false means import anything achieving good LoS
    #[wasm_bindgen(js_name = importExistingRoutes)]
    pub fn import_existing_routes_wasm(&mut self, only_some_infra_types: bool) {
        self.import_existing_routes(only_some_infra_types);
    }

    // TODO Unused
    #[wasm_bindgen(js_name = importCoherentNetwork)]
    pub fn import_coherent_network_wasm(&mut self) {
        self.import_coherent_network();
    }

    #[wasm_bindgen(js_name = importArterialRoads)]
    pub fn import_arterial_roads_wasm(&mut self) {
        self.import_arterial_roads();
    }

    // TODO Except greenspaces
    #[wasm_bindgen(js_name = getPOIs)]
    pub fn get_pois(&self) -> Result<String, JsValue> {
        // TODO Some kind of caching would make this nicer
        let roads = self.get_reachable_network();

        let mut features = Vec::new();
        for (idx, poi) in self.schools.iter().enumerate() {
            features.push(poi.to_gj(&self.graph.mercator, roads.covers(poi.road), idx));
        }
        for (idx, poi) in self.gp_hospitals.iter().enumerate() {
            features.push(poi.to_gj(&self.graph.mercator, roads.covers(poi.road), idx));
        }
        for (idx, poi) in self.railway_stations.iter().enumerate() {
            features.push(poi.to_gj(&self.graph.mercator, roads.covers(poi.road), idx));
        }

        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features,
        })
        .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getGreenspaces)]
    pub fn get_greenspaces(&self) -> Result<String, JsValue> {
        // TODO Some kind of caching would make this nicer
        let roads = self.get_reachable_network();

        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .greenspaces
                .iter()
                .enumerate()
                .flat_map(|(idx, x)| x.to_gj(&self.graph.mercator, roads.covers_any(&x.roads), idx))
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

    #[wasm_bindgen(js_name = getTownCentrePoints)]
    pub fn get_town_centre_points(&self) -> Result<String, JsValue> {
        serde_json::to_string(&FeatureCollection {
            bbox: None,
            foreign_members: None,
            features: self
                .town_centres
                .iter()
                .map(|x| {
                    self.graph
                        .mercator
                        .to_wgs84_gj(&x.polygon.centroid().unwrap())
                })
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

    /// A JSONified list of (name, bounds)
    #[wasm_bindgen(js_name = getSettlementLocations)]
    pub fn get_settlement_locations(&self) -> Result<String, JsValue> {
        let mut list: Vec<(String, [f64; 4])> = Vec::new();
        let mut untitled = 0;
        for s in &self.settlements {
            let name = match s.name.clone() {
                Some(x) => x,
                None => {
                    untitled += 1;
                    format!("Untitled settlement {untitled}")
                }
            };
            let mut bbox: Rect = s.polygon.bounding_rect().unwrap().into();
            self.graph.mercator.to_wgs84_in_place(&mut bbox);
            list.push((
                name,
                [bbox.min().x, bbox.min().y, bbox.max().x, bbox.max().y],
            ));
        }
        list.sort_by_key(|(name, _)| name.clone());
        serde_json::to_string(&list).map_err(err_to_js)
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

    #[wasm_bindgen(js_name = getConnectedComponents)]
    pub fn get_connected_components_wasm(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.get_connected_components()).map_err(err_to_js)
    }

    /// From exactly two waypoints, return a list of extra intermediate nodes and a boolean to
    /// indicate if they're snappable or not.
    #[wasm_bindgen(js_name = getExtraNodes)]
    pub fn get_extra_nodes_wasm(
        &self,
        raw_waypt1: JsValue,
        raw_waypt2: JsValue,
        major_snap_threshold: Option<f64>,
    ) -> Result<String, JsValue> {
        let mut waypt1: Waypoint = serde_wasm_bindgen::from_value(raw_waypt1)?;
        let mut waypt2: Waypoint = serde_wasm_bindgen::from_value(raw_waypt2)?;
        self.to_mercator(&mut waypt1.point);
        self.to_mercator(&mut waypt2.point);
        self.get_extra_nodes(waypt1, waypt2, major_snap_threshold)
            .map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getMajorJunctions)]
    pub fn get_major_junctions(&self) -> Result<String, JsValue> {
        let mut features = Vec::new();
        for i in &self.graph.intersections {
            if crate::is_major_junction(i, &self.highways) {
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

    #[wasm_bindgen(js_name = getTownCentreRoutes)]
    pub fn get_town_centre_routes_wasm(&mut self) -> Result<String, JsValue> {
        let mut timer = Timer::new("recalculate quiet router", None);
        if !self.quiet_router_ok {
            self.recalculate_quiet_router(&mut timer);
        }

        self.get_town_centre_routes().map_err(err_to_js)
    }

    #[wasm_bindgen(js_name = getNetworkLengths)]
    pub fn get_network_lengths_wasm(&self) -> Result<String, JsValue> {
        self.get_network_lengths().map_err(err_to_js)
    }

    fn to_mercator(&self, pt: &mut [f64; 2]) {
        let c: Coord = Coord { x: pt[0], y: pt[1] };
        let out = self.graph.mercator.pt_to_mercator(c);
        *pt = [out.x, out.y];
    }

    fn get_poi_roads(&self, kind: &str, idx: usize) -> Result<HashSet<RoadID>, JsValue> {
        match kind {
            "schools" => Ok([self.schools[idx].road].into()),
            "gp_hospitals" => Ok([self.gp_hospitals[idx].road].into()),
            "railway_stations" => Ok([self.railway_stations[idx].road].into()),
            "greenspaces" => Ok(self.greenspaces[idx].roads.clone()),
            "town_centres" => Ok(self.town_centres[idx].roads.clone()),
            "settlements" => Ok(self.settlements[idx].roads.clone()),
            _ => Err(err_to_js(format!(
                "debug_reachable_path_wasm got bad kind {kind}"
            ))),
        }
    }
}

#[derive(Deserialize)]
struct EvaluateRouteRequest {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    breakdown: String,
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
