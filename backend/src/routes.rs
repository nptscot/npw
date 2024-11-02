use std::collections::HashSet;

use anyhow::Result;
use geojson::{feature::Id, Feature, GeoJson, Geometry};
use graph::{Graph, Road, RoadID};
use serde::Serialize;

use crate::{InfraType, MapModel, Route};

impl MapModel {
    /// Returns the route ID
    pub fn set_route(&mut self, edit_id: Option<usize>, route: Route) -> Result<usize> {
        if let Some(id) = edit_id {
            if self.routes.remove(&id).is_none() {
                bail!("Unknown route {id}");
            }
        }

        // Check for overlaps
        let used_roads: HashSet<RoadID> = self
            .routes
            .values()
            .flat_map(|route| route.roads.clone())
            .collect();
        if route.roads.iter().any(|r| used_roads.contains(r)) {
            bail!("Another route already crosses the same road");
        }

        let id = match edit_id {
            Some(id) => id,
            None => {
                let id = self.id_counter;
                self.id_counter += 1;
                id
            }
        };
        self.routes.insert(id, route);
        Ok(id)
    }

    pub fn delete_route(&mut self, id: usize) -> Result<()> {
        if self.routes.remove(&id).is_some() {
            return Ok(());
        }
        bail!("Unknown route {id}");
    }

    pub fn clear_all_routes(&mut self) {
        self.routes.clear();
    }

    pub fn to_routes_gj(&self) -> GeoJson {
        let mut features = Vec::new();
        for (id, route) in &self.routes {
            let mut f = route.feature.clone();
            f.id = Some(Id::Number((*id).into()));
            f.set_property("name", route.name.clone());
            f.set_property("notes", route.notes.clone());
            f.set_property(
                "infra_type",
                serde_json::to_value(&route.infra_type).unwrap(),
            );
            features.push(f);
        }
        GeoJson::from(features)
    }

    /// Returns the number of edits
    pub fn import_existing_routes(&mut self) -> usize {
        let used_roads: HashSet<RoadID> = self
            .routes
            .values()
            .flat_map(|route| route.roads.clone())
            .collect();

        let mut changes = 0;
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let road_id = RoadID(idx);
            if used_roads.contains(&road_id) {
                continue;
            }
            let Some(infra_type) = crate::existing::classify(&road.osm_tags) else {
                continue;
            };
            if !matches!(
                infra_type,
                InfraType::SegregatedWide | InfraType::OffRoad | InfraType::SegregatedNarrow,
            ) {
                continue;
            }

            // TODO We could group contiguous segments into larger routes, to be nicer
            let route = Route {
                feature: make_route_snapper_feature(&self.graph, road),
                name: road
                    .osm_tags
                    .get("name")
                    .cloned()
                    .unwrap_or_else(String::new),
                notes: "imported from existing network".to_string(),
                roads: vec![road_id],
                infra_type,
            };
            let route_id = self.id_counter;
            self.id_counter += 1;
            self.routes.insert(route_id, route);
            changes += 1;
        }

        changes
    }
}

// Mimic enough of what the route snapper creates, so the segment can be edited in the web app
fn make_route_snapper_feature(graph: &Graph, road: &Road) -> Feature {
    let mut f = Feature::from(Geometry::from(&graph.mercator.to_wgs84(&road.linestring)));
    let pt1 = graph
        .mercator
        .to_wgs84(&graph.intersections[road.src_i.0].point);
    let pt2 = graph
        .mercator
        .to_wgs84(&graph.intersections[road.dst_i.0].point);

    f.set_property(
        "waypoints",
        serde_json::Value::Array(vec![
            serde_json::to_value(&RouteWaypoint {
                lon: pt1.x(),
                lat: pt1.y(),
                snapped: true,
            })
            .unwrap(),
            serde_json::to_value(&RouteWaypoint {
                lon: pt2.x(),
                lat: pt2.y(),
                snapped: true,
            })
            .unwrap(),
        ]),
    );
    f.set_property(
        "full_path",
        serde_json::Value::Array(vec![
            serde_json::to_value(&JsonNode {
                snapped: road.src_i.0 as u32,
            })
            .unwrap(),
            serde_json::to_value(&JsonNode {
                snapped: road.dst_i.0 as u32,
            })
            .unwrap(),
        ]),
    );

    f
}

#[derive(Serialize)]
struct RouteWaypoint {
    lon: f64,
    lat: f64,
    snapped: bool,
}

#[derive(Serialize)]
struct JsonNode {
    snapped: u32,
}
