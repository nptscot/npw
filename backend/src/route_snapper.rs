use route_snapper_graph::{Edge, NodeID, RouteSnapperMap};

use geo::LineString;
use geojson::Feature;
use graph::{Direction, Graph, RoadID};
use serde::Serialize;

use crate::{Dir, MapModel};

impl MapModel {
    pub fn to_route_snapper_graph(&self) -> RouteSnapperMap {
        let profile = self.graph.profile_names["bicycle"];

        let mut nodes = Vec::new();
        for i in &self.graph.intersections {
            nodes.push(self.graph.mercator.to_wgs84(&i.point).into());
        }

        let mut edges = Vec::new();
        for r in &self.graph.roads {
            if r.access[profile.0] == Direction::None {
                continue;
            }
            edges.push(Edge {
                node1: NodeID(r.src_i.0 as u32),
                node2: NodeID(r.dst_i.0 as u32),
                geometry: self.graph.mercator.to_wgs84(&r.linestring),
                name: None,

                // Isn't serialized, doesn't matter
                length_meters: 0.0,
                forward_cost: None,
                backward_cost: None,
            });
        }

        RouteSnapperMap {
            nodes,
            edges,
            override_forward_costs: Vec::new(),
            override_backward_costs: Vec::new(),
        }
    }
}

// Mimic enough of what the route snapper creates, so the segment can be edited in the web app
pub fn make_route_snapper_feature(
    graph: &Graph,
    ids: &[(RoadID, Dir)],
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

    let mut f = graph.mercator.to_wgs84_gj(linestring);

    // We don't know what waypoints we could leave out without doing some kind of iterative
    // approach. For now, just include all of them.
    let waypoints = intersections
        .iter()
        .map(|i| {
            let pt = graph.mercator.to_wgs84(&graph.intersections[i.0].point);
            serde_json::to_value(&RouteWaypoint {
                lon: trim_lon_lat(pt.x()),
                lat: trim_lon_lat(pt.y()),
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

// Per https://datatracker.ietf.org/doc/html/rfc7946#section-11.2, 6 decimal places (10cm) is
// plenty of precision
fn trim_lon_lat(x: f64) -> f64 {
    (x * 10e6).round() / 10e6
}
