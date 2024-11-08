use std::collections::HashSet;

use anyhow::Result;
use geo::LineString;
use geojson::{feature::Id, Feature, GeoJson, Geometry};
use graph::{Graph, RoadID};
use serde::Serialize;

use crate::join_lines::{Dir, KeyedLineString};
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
        self.recalculate_after_edits();
        Ok(id)
    }

    pub fn delete_route(&mut self, id: usize) -> Result<()> {
        if self.routes.remove(&id).is_some() {
            self.recalculate_after_edits();
            return Ok(());
        }
        bail!("Unknown route {id}");
    }

    pub fn clear_all_routes(&mut self) {
        self.routes.clear();
        self.recalculate_after_edits();
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
        // Find individual segments to import
        let mut pieces = Vec::new();
        let used_roads: HashSet<RoadID> = self
            .routes
            .values()
            .flat_map(|route| route.roads.clone())
            .collect();
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

            pieces.push(KeyedLineString {
                linestring: road.linestring.clone(),
                ids: vec![(road_id, Dir::Forwards)],
                key: infra_type,
            });
        }

        // Group them in hopefully meaningful chunks
        // TODO Could try more aggressive joining after this, but this one seems to work fine so
        // far. Although oddly it seems to handle more than just degree 2...
        pieces = crate::join_lines::collapse_degree_2(pieces);
        let changes = pieces.len();

        for line in pieces {
            let route = Route {
                feature: make_route_snapper_feature(&self.graph, &line.ids, &line.linestring),
                // Pick the first name
                // TODO Does this short-circuit?
                name: line
                    .ids
                    .iter()
                    .filter_map(|(r, _)| self.graph.roads[r.0].osm_tags.get("name").cloned())
                    .next()
                    .unwrap_or_else(String::new),
                notes: "imported from existing network".to_string(),
                roads: line.ids.into_iter().map(|(r, _)| r).collect(),
                infra_type: line.key,
            };
            let route_id = self.id_counter;
            self.id_counter += 1;
            self.routes.insert(route_id, route);
        }

        self.recalculate_after_edits();
        changes
    }
}

// Mimic enough of what the route snapper creates, so the segment can be edited in the web app
fn make_route_snapper_feature(
    graph: &Graph,
    ids: &Vec<(RoadID, Dir)>,
    linestring: &LineString,
) -> Feature {
    let mut intersections = Vec::new();
    for (r, dir) in ids {
        let road = &graph.roads[r.0];
        if matches!(dir, Dir::Forwards) {
            intersections.push(road.src_i);
            intersections.push(road.dst_i);
        } else {
            intersections.push(road.dst_i);
            intersections.push(road.src_i);
        }
    }
    intersections.dedup();

    let mut f = Feature::from(Geometry::from(&graph.mercator.to_wgs84(linestring)));

    // We don't know what waypoints we could leave out without doing some kind of iterative
    // approach. For now, just include all of them.
    let waypoints = intersections
        .iter()
        .map(|i| {
            let pt = graph.mercator.to_wgs84(&graph.intersections[i.0].point);
            serde_json::to_value(&RouteWaypoint {
                lon: pt.x(),
                lat: pt.y(),
                snapped: true,
            })
            .unwrap()
        })
        .collect();
    f.set_property("waypoints", serde_json::Value::Array(waypoints));

    let full_path = intersections
        .iter()
        .map(|i| {
            serde_json::to_value(&JsonNode {
                snapped: i.0 as u32,
            })
            .unwrap()
        })
        .collect();
    f.set_property("full_path", serde_json::Value::Array(full_path));

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
