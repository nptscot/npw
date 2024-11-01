use std::time::Duration;

use anyhow::Result;
use geo::{EuclideanLength, LineString};
use geojson::{Feature, FeatureCollection, Geometry};
use graph::Direction;
use utils::Tags;

use crate::{InfraType, MapModel};

/// This determines what's in the graph
pub fn bicycle_profile(tags: &Tags, linestring: &LineString) -> (Direction, Duration) {
    // This is somewhat based on
    // https://github.com/nptscot/osmactive/blob/b08d91b310187c6b344d3682e040e47ce2519be1/R/osmactive.R#L133-L316,
    // but the purpose is different -- unless a road can't be modified, then it still belongs in
    // the graph.

    // Exclude dedicated sidewalks; they're almost always parallel to a road that should be
    // edited instead
    if tags.is("footway", "sidewalk") {
        return (Direction::None, Duration::ZERO);
    }
    // These don't have the potential to become part of a network
    if tags.is("highway", "steps") {
        return (Direction::None, Duration::ZERO);
    }

    // 10mph
    let speed = 4.4704;
    let cost = Duration::from_secs_f64(linestring.euclidean_length() / speed);
    (Direction::Both, cost)
}

impl MapModel {
    pub fn classify_existing_network(&self) -> Result<String> {
        let mut features = Vec::new();
        for road in &self.graph.roads {
            if let Some(infra_type) = classify(&road.osm_tags) {
                let mut f = Feature::from(Geometry::from(
                    &self.graph.mercator.to_wgs84(&road.linestring),
                ));
                f.set_property("infra_type", serde_json::to_value(infra_type)?);
                f.set_property("way", format!("{}", road.way));
                features.push(f);
            }
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
    }
}

// https://github.com/nptscot/osmactive/blob/main/R/osmactive.R is a reference implementation.
// Don't include MixedTraffic or Unknown. If a road has two types of infrastructure in each
// direction, return the stronger case.
//
// TODO This is only a partial implementation
pub fn classify(tags: &Tags) -> Option<InfraType> {
    if tags.is_any("highway", vec!["cycleway", "pedestrian", "footway", "path"]) {
        // TODO maybe regex
        if let Some(name) = tags.get("name") {
            // TODO case sensitivity?
            if ["Path", "Towpath", "Railway", "Trail"]
                .iter()
                .any(|x| name.contains(x))
            {
                return Some(InfraType::OffRoad);
            }
        }
    }

    if tags.is("highway", "cycleway")
        || is_any_key(
            tags,
            vec![
                "cycleway",
                "cycleway:left",
                "cycleway:right",
                "cycleway:both",
            ],
            "track",
        )
    {
        return Some(if is_wide_track(tags) {
            InfraType::SegregatedWide
        } else {
            InfraType::SegregatedNarrow
        });
    }

    if tags.is_any("highway", vec!["footway", "path"])
        && tags.is_any("bicycle", vec!["yes", "designated"])
    {
        if tags.is("segregated", "yes") {
            // TODO Not sure
            return Some(InfraType::OffRoad);
        } else {
            return Some(InfraType::SharedFootway);
        }
    }

    // TODO combo is_any method for keys and values?
    for key in [
        "cycleway",
        "cycleway:left",
        "cycleway:right",
        "cycleway:both",
    ] {
        if tags.is_any(key, vec!["lane", "share_busway"]) {
            return Some(InfraType::CycleLane);
        }
    }

    None
}

// Over 2m?
fn is_wide_track(tags: &Tags) -> bool {
    for key in ["width", "est_width"] {
        if let Some(value) = tags.get(key) {
            // TODO better parsing, sometimes can be a range
            if let Ok(width) = value.parse::<f64>() {
                return width > 2.0;
            }
        }
    }
    false
}

// TODO Upstream
fn is_any_key(tags: &Tags, keys: Vec<&'static str>, value: &str) -> bool {
    keys.iter().any(|k| tags.is(k, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify() {
        let mut ok = true;
        for (input, expected) in [
            // Examples of each class
            (
                vec![
                    "highway=cycleway",
                    "cycleway=track",
                    "foot=no",
                    "width=5",
                    "lanes=2",
                    "oneway=no",
                    "surface=asphalt",
                    "lit=yes",
                    "lcn=yes",
                ],
                Some(InfraType::SegregatedWide),
            ),
            (
                vec![
                    "highway=cycleway",
                    "cycleway=track",
                    "foot=yes",
                    "width=3",
                    "segregated=no",
                    "lanes=2",
                    "oneway=no",
                    "surface=asphalt",
                    "lit=yes",
                ],
                Some(InfraType::OffRoad),
            ),
            (
                vec![
                    "highway=cycleway",
                    "cycleway=track",
                    "foot=no",
                    "width=2",
                    "oneway=no",
                    "surface=asphalt",
                    "lit=yes",
                    "lcn=yes",
                ],
                Some(InfraType::SegregatedNarrow),
            ),
            (
                vec![
                    "highway=footway",
                    "bicycle=yes",
                    "width=2.5",
                    "segregated=no",
                    "oneway=no",
                    "surface=asphalt",
                ],
                Some(InfraType::SharedFootway),
            ),
            (
                vec![
                    "highway=primary",
                    "cycleway=lane",
                    "width=1.5",
                    "lit=yes",
                    "cycleway:separation=no",
                ],
                Some(InfraType::CycleLane),
            ),
            // This is an example of MixedTraffic for route costing purposes, but for
            // classification, we ignore it
            (vec!["highway=residential"], None),
            // Regression tests
            (vec!["highway=path", "name=Eastwood Trail"], None),
            (
                vec![
                    "highway=path",
                    "name=Waulkmills Nature Trail",
                    "bicycle=designated",
                ],
                Some(InfraType::OffRoad),
            ),
        ] {
            let mut tags = Tags::empty();
            for kv in &input {
                let parts = kv.split("=").collect::<Vec<_>>();
                tags.insert(parts[0], parts[1]);
            }
            let actual = classify(&tags);
            if actual != expected {
                println!("For {input:?}, expected {expected:?} but got {actual:?}\n");
                ok = false;
            }
        }

        if !ok {
            panic!("Some cases failed");
        }
    }
}
