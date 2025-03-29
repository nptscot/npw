use anyhow::Result;

use geo::{Coord, LineString, Point};
use geojson::Feature;
use graph::{Graph, IntersectionID, PathStep, RoadID};
use serde::{Deserialize, Serialize};

use crate::{Dir, MapModel};

impl MapModel {
    pub fn snap_route(&self, waypoints: Vec<Waypoint>, prefer_major: bool) -> Result<String> {
        let (roads, linestring) = self.waypoints_to_path(&waypoints, prefer_major);

        // TODO Should we change the input_waypoints to their snapped position?

        let mut feature = self.graph.mercator.to_wgs84_gj(&linestring);

        let props = SerializeRouteProps { roads, waypoints };
        feature.properties = Some(
            serde_json::to_value(&props)
                .unwrap()
                .as_object()
                .unwrap()
                .clone(),
        );

        // TODO Don't we need to set all the RouteProps?

        Ok(serde_json::to_string(&feature)?)
    }

    pub fn get_extra_nodes(
        &self,
        waypt1: Waypoint,
        waypt2: Waypoint,
        prefer_major: bool,
    ) -> Result<String> {
        // TODO If both waypoints aren't snapped, just return one extra node in the middle
        assert!(waypt1.snapped);
        assert!(waypt2.snapped);

        let (roads, _) = self.waypoints_to_path(&vec![waypt1, waypt2], prefer_major);
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
        prefer_major: bool,
    ) -> (Vec<(RoadID, Dir)>, LineString) {
        let profile = self.graph.profile_names["bicycle_direct"];
        let mut roads = Vec::new();
        let mut pts: Vec<Coord> = Vec::new();

        for pair in waypts.windows(2) {
            // Always add every waypoint
            pts.push(pair[0].point.into());

            if pair[0].snapped && pair[1].snapped {
                let start = self.snap_to_intersection(pair[0].point.into(), prefer_major);
                let end = self.snap_to_intersection(pair[1].point.into(), prefer_major);

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

    fn snap_to_intersection(&self, pt: Point, prefer_major: bool) -> IntersectionID {
        if prefer_major {
            let i = self
                .closest_intersection_major
                .nearest_neighbor(&pt)
                .unwrap()
                .data;
            // TODO Check distance
            return i;
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

// Mimic the same output as snap_route, generating it from a different description.
pub fn make_route_snapper_feature(
    graph: &Graph,
    roads: &[(RoadID, Dir)],
    linestring: &LineString,
) -> Feature {
    let intersections = roads_to_intersections(graph, roads);

    let waypoints = find_minimal_waypoints(graph, roads, &intersections)
        .into_iter()
        .map(|i| {
            let pt = graph.mercator.to_wgs84(&graph.intersections[i.0].point);
            Waypoint {
                point: [trim_lon_lat(pt.x()), trim_lon_lat(pt.y())],
                snapped: true,
            }
        })
        .collect();

    let mut f = graph.mercator.to_wgs84_gj(linestring);
    let props = SerializeRouteProps {
        roads: roads.to_vec(),
        waypoints,
    };
    f.properties = Some(
        serde_json::to_value(&props)
            .unwrap()
            .as_object()
            .unwrap()
            .clone(),
    );

    f
}

// From the full sequence of intersections in a route, find the snapped waypoints that will
// reproduce this using the bicycle_direct profile.
fn find_minimal_waypoints(
    graph: &Graph,
    steps: &[(RoadID, Dir)],
    intersections: &Vec<IntersectionID>,
) -> Vec<IntersectionID> {
    use crate::routes::{end_pos, start_pos};

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
    let i1 = intersections[0];
    let i2 = *intersections.last().unwrap();
    let start = start_pos(steps[0], graph);
    let end = end_pos(*steps.last().unwrap(), graph);
    assert_eq!(i1, start.intersection);
    assert_eq!(i2, end.intersection);

    if let Ok(route) = graph.routers[profile.0].route(graph, start, end) {
        if path_steps == route.steps {
            return vec![i1, i2];
        }
    }

    // TODO Try some kind of iterative / binary search approach to pruning

    // Give up and just include them all
    intersections.iter().cloned().collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Waypoint {
    pub point: [f64; 2],
    snapped: bool,
}

// Per https://datatracker.ietf.org/doc/html/rfc7946#section-11.2, 6 decimal places (10cm) is
// plenty of precision
fn trim_lon_lat(x: f64) -> f64 {
    (x * 10e6).round() / 10e6
}

// TODO Maybe temporary
#[derive(Serialize)]
struct SerializeRouteProps {
    roads: Vec<(RoadID, Dir)>,
    waypoints: Vec<Waypoint>,
}
