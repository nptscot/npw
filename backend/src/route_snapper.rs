use anyhow::Result;

use geo::{Coord, Distance, Euclidean, LineString, Point};
use graph::{Graph, IntersectionID, PathStep, RoadID};

use crate::{Dir, MapModel, Waypoint};

impl MapModel {
    pub fn get_extra_nodes(
        &self,
        waypt1: Waypoint,
        waypt2: Waypoint,
        major_snap_threshold: Option<f64>,
    ) -> Result<String> {
        // TODO If both waypoints aren't snapped, just return one extra node in the middle
        assert!(waypt1.snapped);
        assert!(waypt2.snapped);

        let (roads, _) = self.waypoints_to_path(&vec![waypt1, waypt2], major_snap_threshold);
        let intersections = roads_to_intersections(&self.graph, &roads);

        let mut extra_nodes: Vec<(f64, f64, bool)> = Vec::new();
        for (idx, i) in intersections.iter().enumerate() {
            // Skip the first and last, so only intermediate nodes are returned
            if idx == 0 || idx == intersections.len() - 1 {
                continue;
            }

            let pt = self
                .graph
                .mercator
                .pt_to_wgs84(self.graph.intersections[i.0].point.into());
            extra_nodes.push((pt.x, pt.y, true));
        }

        Ok(serde_json::to_string(&extra_nodes)?)
    }

    /// Turns waypoints into a full path description and a LineString
    pub fn waypoints_to_path(
        &self,
        waypts: &Vec<Waypoint>,
        major_snap_threshold: Option<f64>,
    ) -> (Vec<(RoadID, Dir)>, LineString) {
        let profile = self.graph.profile_names["bicycle_direct"];
        let mut roads = Vec::new();
        let mut pts: Vec<Coord> = Vec::new();

        for pair in waypts.windows(2) {
            // Always add every waypoint
            pts.push(pair[0].point.into());

            if pair[0].snapped && pair[1].snapped {
                let start = self.snap_to_intersection(pair[0].point.into(), major_snap_threshold);
                let end = self.snap_to_intersection(pair[1].point.into(), major_snap_threshold);

                if let Ok(route) = self.graph.routers[profile.0].route_between_intersections(
                    &self.graph,
                    start,
                    end,
                ) {
                    pts.extend(route.linestring(&self.graph).into_inner());

                    for step in route.steps {
                        match step {
                            PathStep::Road { road, forwards } => {
                                roads.push((
                                    road,
                                    if forwards {
                                        Dir::Forwards
                                    } else {
                                        Dir::Backwards
                                    },
                                ));
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                // If the points are disconnected in the graph, just act like there's a freehand
                // line between them. It's better than breaking.
                // (We don't need to do anything here -- the other point will get added)
                // TODO Revisit
            }
        }
        // Always add the last if it's different
        if let Some(last) = waypts.last() {
            if !last.snapped {
                pts.push(last.point.into());
            }
        }

        pts.dedup();
        (roads, LineString::new(pts))
    }

    /// When `major_snap_threshold` is specified, then major intersections should be used, as long
    /// as they are within this many meters of the pt.
    pub fn snap_to_intersection(
        &self,
        pt: Point,
        major_snap_threshold: Option<f64>,
    ) -> IntersectionID {
        if let Some(threshold) = major_snap_threshold {
            let i = self
                .closest_intersection_major
                .nearest_neighbor(&pt)
                .unwrap()
                .data;

            let dist = Euclidean.distance(pt, self.graph.intersections[i.0].point);
            if dist <= threshold {
                return i;
            }
        }

        self.closest_intersection_all
            .nearest_neighbor(&pt)
            .unwrap()
            .data
    }
}

fn roads_to_intersections(graph: &Graph, roads: &[(RoadID, Dir)]) -> Vec<IntersectionID> {
    // TODO If we retain the graph::Route, we get this for free
    let mut intersections = Vec::new();
    for (r, dir) in roads {
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
    intersections
}

/// Returns waypoints in WGS84
pub fn roads_to_waypoints(graph: &Graph, roads: &[(RoadID, Dir)]) -> Vec<Waypoint> {
    let intersections = roads_to_intersections(graph, roads);
    find_minimal_waypoints(graph, roads, &intersections)
        .into_iter()
        .map(|i| {
            let pt = graph.mercator.to_wgs84(&graph.intersections[i.0].point);
            Waypoint {
                point: [trim_lon_lat(pt.x()), trim_lon_lat(pt.y())],
                snapped: true,
            }
        })
        .collect()
}

// From the full sequence of intersections in a route, find the snapped waypoints that will
// reproduce this using the bicycle_direct profile.
fn find_minimal_waypoints(
    graph: &Graph,
    steps: &[(RoadID, Dir)],
    intersections: &Vec<IntersectionID>,
) -> Vec<IntersectionID> {
    // TODO Really rethink the route snapper backend now, because the translation back and
    // forth is getting silly
    let path_steps: Vec<PathStep> = steps
        .iter()
        .cloned()
        .map(|(road, dir)| PathStep::Road {
            road,
            forwards: matches!(dir, Dir::Forwards),
        })
        .collect();

    let profile = graph.profile_names["bicycle_direct"];

    // Try the optimistic, simple approach first -- just the first and last point
    let simple = vec![intersections[0], *intersections.last().unwrap()];
    if let Ok(route) =
        graph.routers[profile.0].route_between_many_intersections(graph, simple.clone())
    {
        if path_steps == route.steps {
            return simple;
        }
    }

    let mut current: Vec<IntersectionID> = intersections.iter().cloned().collect();
    let mut idx = 1;
    // Try removing one middle waypoint at a time
    while idx < current.len() - 1 {
        let removed = current.remove(idx);
        if let Ok(route) =
            graph.routers[profile.0].route_between_many_intersections(graph, current.clone())
        {
            if path_steps == route.steps {
                // Keep the same idx
                continue;
            }
        }
        // Put it back and move on
        current.insert(idx, removed);
        idx += 1;
    }
    current
}

// Per https://datatracker.ietf.org/doc/html/rfc7946#section-11.2, 6 decimal places (10cm) is
// plenty of precision
fn trim_lon_lat(x: f64) -> f64 {
    (x * 10e6).round() / 10e6
}
