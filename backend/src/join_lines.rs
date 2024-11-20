use std::collections::{BTreeSet, HashMap};

use geo::{Coord, Euclidean, Length, LineString};
use graph::RoadID;
use petgraph::graphmap::UnGraphMap;

use crate::InfraType;

// TODO For simplicty right now, hardcodes an ID and key type. Make generic later.
// TODO Upstream in geo or utils

#[derive(Clone)]
pub enum Dir {
    Forwards,
    Backwards,
}

/// A linestring with a list of IDs in order and some key
pub struct KeyedLineString {
    pub linestring: LineString,
    pub ids: Vec<(RoadID, Dir)>,
    pub key: InfraType,
}

// Also contains the key. Linestrings with different keys are effectively disconnected.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HashedPoint(isize, isize, InfraType);

#[derive(Clone, Copy, PartialEq, Eq)]
struct EdgeIdx(usize);

impl HashedPoint {
    fn new(pt: Coord, key: InfraType) -> Self {
        // cm precision
        Self((pt.x * 100.0) as isize, (pt.y * 100.0) as isize, key)
    }
}

/// Takes a bunch of individual linestrings and joins them together greedily. Only joins lines with
/// a matching key.
// TODO Seems to hang, keep it around for later, but unused right now
#[allow(unused)]
pub fn join_linestrings(mut lines: Vec<KeyedLineString>) -> Vec<KeyedLineString> {
    loop {
        log::info!("join_linestrings: {} lines left", lines.len());
        // Build a graph from the lines
        let mut intersections: BTreeSet<HashedPoint> = BTreeSet::new();
        let mut graph: UnGraphMap<HashedPoint, EdgeIdx> = UnGraphMap::new();

        for (idx, line) in lines.iter().enumerate() {
            let i1 = HashedPoint::new(*line.linestring.0.first().unwrap(), line.key);
            let i2 = HashedPoint::new(*line.linestring.0.last().unwrap(), line.key);
            intersections.insert(i1);
            intersections.insert(i2);
            graph.add_edge(i1, i2, EdgeIdx(idx));
        }

        if let Some(path) = find_longest_path(&graph, &lines, &intersections) {
            lines = join_path(lines, path);
        } else {
            return lines;
        }
    }
}

/// Find all linestrings that meet at one end and join them. Only joins lines with a matching key.
pub fn collapse_degree_2(mut lines: Vec<KeyedLineString>) -> Vec<KeyedLineString> {
    info!("Collapsing degree 2 nodes in {} lines", lines.len());
    // TODO I think this is doable in one pass
    loop {
        let mut intersections: HashMap<HashedPoint, EdgeIdx> = HashMap::new();

        let mut path = None;
        'FIND: for (idx1, line) in lines.iter().enumerate() {
            let i1 = HashedPoint::new(*line.linestring.0.first().unwrap(), line.key);
            let i2 = HashedPoint::new(*line.linestring.0.last().unwrap(), line.key);
            if i1 == i2 {
                continue;
            }

            let idx1 = EdgeIdx(idx1);
            for i in [i1, i2] {
                match intersections.get(&i) {
                    Some(idx2) => {
                        // Don't create a loop though!
                        // TODO Doesn't seem to always work
                        if number_shared_endpoints(line, &lines[idx2.0]) == 1 {
                            path = Some(vec![idx1, *idx2]);
                            break 'FIND;
                        }
                    }
                    None => {
                        intersections.insert(i, idx1);
                    }
                }
            }
        }

        if let Some(path) = path {
            lines = join_path(lines, path);
        } else {
            break;
        }
    }
    lines
}

// Of length > 1
fn find_longest_path(
    graph: &UnGraphMap<HashedPoint, EdgeIdx>,
    edges: &Vec<KeyedLineString>,
    intersections: &BTreeSet<HashedPoint>,
) -> Option<Vec<EdgeIdx>> {
    let mut best_path = Vec::new();
    let mut best_length = 0.0;

    // If we had DAGs, we could try Dijkstra with negative edge weights. For now, just brute-force
    // it -- the graphs should be tiny
    for src in intersections {
        for dst in intersections {
            if src == dst {
                continue;
            }
            if let Some((length, path)) = petgraph::algo::astar(
                graph,
                *src,
                |i| i == *dst,
                |(_, _, idx)| edges[idx.0].linestring.length::<Euclidean>(),
                |_| 0.0,
            ) {
                if path.len() > 2 && length > best_length {
                    best_length = length;
                    best_path = path;
                }
            }
        }
    }

    let mut result = Vec::new();
    for pair in best_path.windows(2) {
        result.push(*graph.edge_weight(pair[0], pair[1]).unwrap());
    }
    if result.is_empty() {
        None
    } else {
        Some(result)
    }
}

// Combines everything in the path, returning a smaller list of lines
fn join_path(lines: Vec<KeyedLineString>, path: Vec<EdgeIdx>) -> Vec<KeyedLineString> {
    // Build up the joined line
    let mut points = Vec::new();
    let mut ids = Vec::new();
    // Trust the caller to only pass in the correct key
    let key = lines[path[0].0].key;

    for idx in &path {
        let mut next_ids = lines[idx.0].ids.clone();
        let mut next_points = lines[idx.0].linestring.clone().into_inner();

        if points.is_empty() {
            points = next_points;
            ids = next_ids;
            continue;
        }
        let pt1 = HashedPoint::new(*points.first().unwrap(), key);
        let pt2 = HashedPoint::new(*points.last().unwrap(), key);
        let pt3 = HashedPoint::new(*next_points.first().unwrap(), key);
        let pt4 = HashedPoint::new(*next_points.last().unwrap(), key);

        if pt1 == pt3 {
            points.reverse();
            points.pop();
            points.extend(next_points);

            ids.reverse();
            flip_direction(&mut ids);
            ids.extend(next_ids);
        } else if pt1 == pt4 {
            next_points.pop();
            next_points.extend(points);
            points = next_points;

            next_ids.extend(ids);
            ids = next_ids;
        } else if pt2 == pt3 {
            points.pop();
            points.extend(next_points);

            ids.extend(next_ids);
        } else if pt2 == pt4 {
            next_points.reverse();
            points.pop();
            points.extend(next_points);

            next_ids.reverse();
            flip_direction(&mut next_ids);
            ids.extend(next_ids);
        } else {
            unreachable!()
        }
    }

    let mut result = vec![KeyedLineString {
        linestring: LineString::new(points),
        ids,
        key,
    }];

    // Leftovers
    for (i, line) in lines.into_iter().enumerate() {
        if !path.contains(&EdgeIdx(i)) {
            result.push(line);
        }
    }
    result
}

fn number_shared_endpoints(line1: &KeyedLineString, line2: &KeyedLineString) -> usize {
    let mut set = BTreeSet::new();
    set.insert(HashedPoint::new(
        *line1.linestring.0.first().unwrap(),
        line1.key,
    ));
    set.insert(HashedPoint::new(
        *line1.linestring.0.last().unwrap(),
        line1.key,
    ));
    set.insert(HashedPoint::new(
        *line2.linestring.0.first().unwrap(),
        line2.key,
    ));
    set.insert(HashedPoint::new(
        *line2.linestring.0.last().unwrap(),
        line2.key,
    ));
    4 - set.len()
}

fn flip_direction(ids: &mut Vec<(RoadID, Dir)>) {
    for pair in ids {
        pair.1 = match pair.1 {
            Dir::Forwards => Dir::Backwards,
            Dir::Backwards => Dir::Forwards,
        }
    }
}
