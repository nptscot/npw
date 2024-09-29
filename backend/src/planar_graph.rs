use std::collections::{HashMap, HashSet};

use geo::{Coord, LineString, Point};
use geojson::{Feature, FeatureCollection, Geometry};
use utils::Mercator;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct HashedPoint(isize, isize);

impl HashedPoint {
    fn new(pt: Coord) -> Self {
        // cm precision
        Self((pt.x * 100.0) as isize, (pt.y * 100.0) as isize)
    }

    fn to_geo(self) -> Coord {
        Coord {
            x: self.0 as f64 / 100.0,
            y: self.1 as f64 / 100.0,
        }
    }
}

pub fn get_faces(mercator: &Mercator, input: &Vec<LineString>) -> String {
    // Repeat every edge in both directions
    let mut linestrings = Vec::new();
    for ls in input {
        linestrings.push(ls.clone());
        let mut rev = ls.clone();
        rev.0.reverse();
        linestrings.push(rev);
    }

    // Build a graph from these. Edges are identified by index.
    let mut edges_per_node: HashMap<HashedPoint, Vec<usize>> = HashMap::new();
    for (idx, linestring) in linestrings.iter().enumerate() {
        let i1 = HashedPoint::new(*linestring.0.first().unwrap());
        let i2 = HashedPoint::new(*linestring.0.last().unwrap());
        edges_per_node.entry(i1).or_insert_with(Vec::new).push(idx);
        edges_per_node.entry(i2).or_insert_with(Vec::new).push(idx);
    }

    // Sort the list of edges at each node so that they're in counter-clockwise order
    for (pt, edges) in &mut edges_per_node {
        sort_edges_ccw(*pt, edges, &linestrings);
    }

    let mut features = Vec::new();
    for (idx, ls) in linestrings.iter().enumerate() {
        let mut f = Feature::from(Geometry::from(&mercator.to_wgs84(ls)));
        f.set_property("idx", idx);
        f.set_property("forwards", idx % 2 == 0);
        features.push(f);
    }

    for (pt, edges) in &edges_per_node {
        let mut f = Feature::from(Geometry::from(&Point::from(
            mercator.pt_to_wgs84(pt.to_geo()),
        )));
        f.set_property("edges", format!("{edges:?}"));
        f.set_property(
            "angles",
            format!(
                "{:?}",
                edges
                    .iter()
                    .map(|idx| {
                        (100.0 * angle_from_endpoint(*pt, &linestrings[*idx])).round() / 100.0
                    })
                    .collect::<Vec<_>>()
            ),
        );

        features.push(f);
    }

    serde_json::to_string(&FeatureCollection {
        features,
        bbox: None,
        foreign_members: None,
    })
    .unwrap()

    /*let mut results = Vec::new();
    let mut visited_edges: HashSet<usize> = HashSet::new();
    for start_idx in 0..linestrings.len() {
        if visited_edges.contains(&start_idx) {
            continue;
        }

        // Look for a cycle starting here
        let mut points = Vec::new();
        let mut current = start_idx;
        loop {
            log::info!("from {start_idx}, at {current}");

            if visited_edges.contains(&current) {
                log::info!("   bug! hit a loop in the wrong way");
                points.dedup();
                results.push(LineString::new(points));
                break;
            }

            visited_edges.insert(current);
            points.extend(linestrings[current].0.clone());

            current = pick_next_ccw_edge(current, &edges_per_node, &linestrings);

            if current == start_idx {
                points.dedup();
                results.push(LineString::new(points));
                break;
            }
        }

        // TODO Only work on one right now
        break;
    }

    results*/
}

fn sort_edges_ccw(endpoint: HashedPoint, edges: &mut Vec<usize>, linestrings: &Vec<LineString>) {
    edges.sort_by_key(|idx| {
        // Make the angle sortable
        (angle_from_endpoint(endpoint, &linestrings[*idx]) * 1000.0) as isize
    });
}

fn angle_from_endpoint(endpoint: HashedPoint, linestring: &LineString) -> f64 {
    let points = &linestring.0;
    // Which end of the linestring starts here?
    let starts_at_endpoint = endpoint == HashedPoint::new(points[0]);
    // Find the angle in degrees from the endpoint to the next point in this line
    let next_pt = if starts_at_endpoint {
        points[1]
    } else {
        points[points.len() - 2]
    };
    let endpoint_geo = endpoint.to_geo();
    let angle_degrees = (next_pt.y - endpoint_geo.y)
        .atan2(next_pt.x - endpoint_geo.x)
        .to_degrees();
    let flip = if starts_at_endpoint { 0.0 } else { 180.0 };
    // Normalize to [0, 360)
    (angle_degrees + flip + 360.0) % 360.0
}

fn pick_next_ccw_edge(
    current: usize,
    edges_per_node: &HashMap<HashedPoint, Vec<usize>>,
    linestrings: &Vec<LineString>,
) -> usize {
    // We always trace forwards on the edges
    let endpoint = HashedPoint::new(*linestrings[current].0.last().unwrap());
    // TODO We don't need to the filter out the edges_per_node for edges originating at this
    // endpoint. If we trust the sorting process, then it'll just happen.
    let edges = &edges_per_node[&endpoint];
    let pos = edges.iter().position(|e| *e == current).unwrap();

    if pos == edges.len() - 1 {
        edges[0]
    } else {
        edges[pos + 1]
    }

    /*if pos == 0 {
        edges[edges.len() - 1]
    } else {
        edges[pos - 1]
    }*/
}
