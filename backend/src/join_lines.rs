use std::collections::{BTreeSet, HashMap};

use geo::{Coord, EuclideanLength, LineString};
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct HashedPoint(isize, isize);

#[derive(Clone, Copy, PartialEq, Eq)]
struct EdgeIdx(usize);

impl HashedPoint {
    fn new(pt: Coord) -> Self {
        // cm precision
        Self((pt.x * 100.0) as isize, (pt.y * 100.0) as isize)
    }
}

// TODO Copied from polygon-width; move to utils or upstream in geo
#[allow(unused)]
pub fn join_linestrings(mut lines: Vec<LineString>) -> Vec<LineString> {
    loop {
        log::info!("join_linestrings: {} lines left", lines.len());
        // Build a graph from the lines
        let mut intersections: BTreeSet<HashedPoint> = BTreeSet::new();
        let mut graph: UnGraphMap<HashedPoint, EdgeIdx> = UnGraphMap::new();

        for (idx, line) in lines.iter().enumerate() {
            let i1 = HashedPoint::new(*line.0.first().unwrap());
            let i2 = HashedPoint::new(*line.0.last().unwrap());
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

/// Find all linestrings that join at one end and join them
pub fn collapse_degree_2(mut lines: Vec<LineString>) -> Vec<LineString> {
    // TODO I think this is doable in one pass
    loop {
        let mut intersections: HashMap<HashedPoint, EdgeIdx> = HashMap::new();

        let mut path = None;
        'FIND: for (idx1, line) in lines.iter().enumerate() {
            let i1 = HashedPoint::new(*line.0.first().unwrap());
            let i2 = HashedPoint::new(*line.0.last().unwrap());
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
    edges: &Vec<LineString>,
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
                |(_, _, idx)| edges[idx.0].euclidean_length(),
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
fn join_path(lines: Vec<LineString>, path: Vec<EdgeIdx>) -> Vec<LineString> {
    // Build up the joined line
    let mut points = Vec::new();
    for idx in &path {
        let mut next = lines[idx.0].clone().into_inner();
        if points.is_empty() {
            points = next;
            continue;
        }
        let pt1 = HashedPoint::new(*points.first().unwrap());
        let pt2 = HashedPoint::new(*points.last().unwrap());
        let pt3 = HashedPoint::new(*next.first().unwrap());
        let pt4 = HashedPoint::new(*next.last().unwrap());

        if pt1 == pt3 {
            points.reverse();
            points.pop();
            points.extend(next);
        } else if pt1 == pt4 {
            next.pop();
            next.extend(points);
            points = next;
        } else if pt2 == pt3 {
            points.pop();
            points.extend(next);
        } else if pt2 == pt4 {
            next.reverse();
            points.pop();
            points.extend(next);
        } else {
            unreachable!()
        }
    }
    let joined = LineString::new(points);
    let mut result = vec![joined];
    for (i, line) in lines.into_iter().enumerate() {
        if !path.contains(&EdgeIdx(i)) {
            result.push(line);
        }
    }
    result
}

fn number_shared_endpoints(line1: &LineString, line2: &LineString) -> usize {
    let mut set = BTreeSet::new();
    set.insert(HashedPoint::new(*line1.0.first().unwrap()));
    set.insert(HashedPoint::new(*line1.0.last().unwrap()));
    set.insert(HashedPoint::new(*line2.0.first().unwrap()));
    set.insert(HashedPoint::new(*line2.0.last().unwrap()));
    4 - set.len()
}
