use anyhow::Result;

use geo::{Coord, Euclidean, Length, Line, LineInterpolatePoint, LineString};
use geojson::Feature;
use graph::{Graph, IntersectionID, PathStep, RoadID};
use serde::{Deserialize, Serialize};

use crate::{Dir, MapModel};

impl MapModel {
    pub fn snap_route(&self, input_waypoints: Vec<InputRouteWaypoint>) -> Result<String> {
        let (path_entries, linestring) = self.get_path_entries(&input_waypoints);
        let length = linestring.length::<Euclidean>();

        let mut feature = self.graph.mercator.to_wgs84_gj(&linestring);
        feature.set_property("length_meters", length);

        let mut full_path = Vec::new();
        for entry in &path_entries {
            match entry {
                PathEntry::SnappedPoint(i) => {
                    full_path.push(
                        serde_json::to_value(&JsonNode {
                            snapped: Some(i.0),
                            free: None,
                        })
                        .unwrap(),
                    );
                }
                PathEntry::FreePoint(pt) => {
                    full_path.push(
                        serde_json::to_value(&JsonNode {
                            snapped: None,
                            free: Some([trim_lon_lat(pt.x), trim_lon_lat(pt.y)]),
                        })
                        .unwrap(),
                    );
                }
                PathEntry::Edge(_, _) => {}
            }
        }
        full_path.dedup();
        feature.set_property("full_path", serde_json::Value::Array(full_path));

        // TODO These used to be corrected to the snapped position. Is that important?
        let mut waypoints = Vec::new();
        for waypt in input_waypoints {
            waypoints.push(
                serde_json::to_value(&OutputRouteWaypoint {
                    lon: trim_lon_lat(waypt.point[0]),
                    lat: trim_lon_lat(waypt.point[1]),
                    snapped: waypt.snapped,
                })
                .unwrap(),
            );
        }
        feature.set_property("waypoints", serde_json::Value::Array(waypoints));

        Ok(serde_json::to_string(&feature)?)
    }

    pub fn get_extra_nodes(
        &self,
        waypt1: InputRouteWaypoint,
        waypt2: InputRouteWaypoint,
    ) -> Result<String> {
        // If both waypoints aren't snapped, just return one extra node in the middle
        if !waypt1.snapped || !waypt2.snapped {
            // If one waypoint is snapped, use its snapped position for finding the middle
            let profile = self.graph.profile_names["bicycle_direct"];
            let pt1 = if waypt1.snapped {
                let i = self
                    .graph
                    .snap_to_road(waypt1.point.into(), profile)
                    .intersection;
                self.graph.intersections[i.0].point
            } else {
                waypt1.point.into()
            };
            let pt2 = if waypt2.snapped {
                let i = self
                    .graph
                    .snap_to_road(waypt2.point.into(), profile)
                    .intersection;
                self.graph.intersections[i.0].point
            } else {
                waypt2.point.into()
            };

            // TODO Need to be more careful with CRS here. But the frontend disabled freehand for
            // now
            let line = Line::new(pt1, pt2);
            if let Some(midpt) = line.line_interpolate_point(0.5) {
                return Ok(serde_json::to_string(&vec![(midpt.x(), midpt.y(), false)])?);
            }
        }

        let (path_entries, _) = self.get_path_entries(&vec![waypt1, waypt2]);

        let mut extra_nodes: Vec<(f64, f64, bool)> = Vec::new();
        for (idx, entry) in path_entries.iter().enumerate() {
            // Skip the first and last, so only intermediate nodes are returned
            if idx == 0 || idx == path_entries.len() - 1 {
                continue;
            }

            if let PathEntry::SnappedPoint(i) = entry {
                let pt = self
                    .graph
                    .mercator
                    .pt_to_wgs84(self.graph.intersections[i.0].point.into());
                extra_nodes.push((pt.x, pt.y, true));
            }
        }

        Ok(serde_json::to_string(&extra_nodes)?)
    }

    /// Turns waypoints into a full path description and a LineString
    fn get_path_entries(&self, waypts: &Vec<InputRouteWaypoint>) -> (Vec<PathEntry>, LineString) {
        let profile = self.graph.profile_names["bicycle_direct"];
        let mut path_entries = Vec::new();
        let mut pts: Vec<Coord> = Vec::new();

        for pair in waypts.windows(2) {
            // Always add every waypoint
            path_entries.push(self.waypt_to_path_entry(&pair[0]));
            pts.push(pair[0].point.into());

            if pair[0].snapped && pair[1].snapped {
                let start = self.graph.snap_to_road(pair[0].point.into(), profile);
                let end = self.graph.snap_to_road(pair[1].point.into(), profile);

                if let Ok(route) = self.graph.routers[profile.0].route(&self.graph, start, end) {
                    // Don't repeat that snapped point
                    assert_eq!(
                        path_entries.pop(),
                        Some(PathEntry::SnappedPoint(start.intersection))
                    );
                    pts.extend(route.linestring(&self.graph).into_inner());
                    for step in route.steps {
                        match step {
                            PathStep::Road { road, forwards } => {
                                path_entries.push(PathEntry::Edge(
                                    road,
                                    if forwards {
                                        Dir::Forwards
                                    } else {
                                        Dir::Backwards
                                    },
                                ));
                                let road = &self.graph.roads[road.0];
                                path_entries.push(PathEntry::SnappedPoint(if forwards {
                                    road.dst_i
                                } else {
                                    road.src_i
                                }));
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
            let add = self.waypt_to_path_entry(last);
            if path_entries.last() != Some(&add) {
                path_entries.push(add);
            }
            if !last.snapped {
                pts.push(last.point.into());
            }
        }

        pts.dedup();
        (path_entries, LineString::new(pts))
    }

    fn waypt_to_path_entry(&self, waypt: &InputRouteWaypoint) -> PathEntry {
        if waypt.snapped {
            let profile = self.graph.profile_names["bicycle_direct"];
            PathEntry::SnappedPoint(
                self.graph
                    .snap_to_road(waypt.point.into(), profile)
                    .intersection,
            )
        } else {
            PathEntry::FreePoint(waypt.point.into())
        }
    }
}

#[derive(PartialEq, Debug)]
enum PathEntry {
    SnappedPoint(IntersectionID),
    FreePoint(Coord),
    Edge(RoadID, Dir),
    // Note we don't need to represent a straight line between snapped or free points here. As we
    // build up the line-string, they'll happen anyway.
}

// Mimic the same output as snap_route, generating it from a different description.
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
    let waypoints = find_minimal_waypoints(graph, ids, &intersections)
        .into_iter()
        .map(|i| {
            let pt = graph.mercator.to_wgs84(&graph.intersections[i.0].point);
            serde_json::to_value(&OutputRouteWaypoint {
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
                snapped: Some(i.0),
                free: None,
            })
            .unwrap()
        })
        .collect();
    f.set_property("full_path", serde_json::Value::Array(full_path));

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

    // Try the optimistic, simple approach first -- just the first and last point
    let i1 = intersections[0];
    let i2 = *intersections.last().unwrap();
    let profile = graph.profile_names["bicycle_direct"];
    let start = start_pos(steps[0], graph);
    let end = end_pos(*steps.last().unwrap(), graph);
    if let Ok(route) = graph.routers[profile.0].route(graph, start, end) {
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
        if path_steps == route.steps {
            return vec![i1, i2];
        }
    }

    // TODO Try some kind of iterative / binary search approach to pruning

    // Give up and just include them all
    intersections.iter().cloned().collect()
}

#[derive(Debug, Deserialize)]
pub struct InputRouteWaypoint {
    pub point: [f64; 2],
    snapped: bool,
}

// TODO Why're there two of these?
#[derive(Serialize)]
struct OutputRouteWaypoint {
    lon: f64,
    lat: f64,
    snapped: bool,
}

#[derive(Serialize)]
struct JsonNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    snapped: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    free: Option<[f64; 2]>,
}

// Per https://datatracker.ietf.org/doc/html/rfc7946#section-11.2, 6 decimal places (10cm) is
// plenty of precision
fn trim_lon_lat(x: f64) -> f64 {
    (x * 10e6).round() / 10e6
}
