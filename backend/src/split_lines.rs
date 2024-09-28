use std::collections::HashMap;

use geo::{Line, LineIntersection, LineLocatePoint, LineString};
use utils::LineSplit;

/// Given a bunch of linestrings, split them into smaller pieces when they cross.
pub fn split_crossing_linestrings(mut input: Vec<LineString>) -> Vec<LineString> {
    let mut all_lines = Vec::new();
    for (idx, linestring) in input.iter().enumerate() {
        for line in linestring.lines() {
            all_lines.push(LineWithData { line, idx });
        }
    }

    // For every linestring, find all fractions where a split is needed
    let mut split_fractions: HashMap<usize, Vec<f64>> = HashMap::new();
    for (obj1, obj2, cross) in geo::sweep::Intersections::<_>::from_iter(all_lines) {
        let LineIntersection::SinglePoint {
            intersection,
            is_proper,
        } = cross
        else {
            continue;
        };
        // Intersections are expected constantly at endpoints, so ignore those
        if !is_proper {
            continue;
        }

        let ls1_fraction = input[obj1.idx]
            .line_locate_point(&intersection.into())
            .unwrap();
        split_fractions
            .entry(obj1.idx)
            .or_insert_with(Vec::new)
            .push(ls1_fraction);

        let ls2_fraction = input[obj2.idx]
            .line_locate_point(&intersection.into())
            .unwrap();
        split_fractions
            .entry(obj2.idx)
            .or_insert_with(Vec::new)
            .push(ls2_fraction);
    }

    // Now split the linestrings at all places needed
    let mut remove_old_linestrings = Vec::new();
    for (idx, fractions) in split_fractions {
        log::info!("splitting {idx} at {fractions:?}");
        for split_ls in input[idx].line_split_many(&fractions).unwrap() {
            let Some(split_ls) = split_ls else {
                // Sometimes the split points are too close together
                // TODO This isn't happening for the example input, but still some lines are
                // disappearing that don't seem to be involved in any crosses
                log::info!("something broke for {idx}. fractions were {fractions:?}");
                continue;
            };
            input.push(split_ls);
        }
        remove_old_linestrings.push(idx);
    }

    // Remove all linestrings we split. Since we're deleting by index/position, do the deletions in
    // reverse
    remove_old_linestrings.reverse();
    for idx in remove_old_linestrings {
        input.remove(idx);
    }

    input
}

#[derive(Clone, Debug)]
struct LineWithData {
    line: Line,
    idx: usize,
}

impl geo::sweep::Cross for LineWithData {
    type Scalar = f64;

    fn line(&self) -> geo::sweep::LineOrPoint<Self::Scalar> {
        self.line.line()
    }
}
