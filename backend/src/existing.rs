use std::time::Duration;

use anyhow::Result;
use geo::{Euclidean, Length, LineString};
use geojson::FeatureCollection;
use graph::Direction;
use utils::Tags;

use crate::{level_of_service::get_speed_mph, InfraType, MapModel};

/// This determines what's in the graph. The cost function is just based on distance.
pub fn bicycle_profile(tags: &Tags, linestring: &LineString) -> (Direction, Duration) {
    // This is somewhat based on
    // https://github.com/nptscot/osmactive/blob/b08d91b310187c6b344d3682e040e47ce2519be1/R/osmactive.R#L133-L316,
    // but the purpose is different -- unless a road can't be modified, then it still belongs in
    // the graph.

    let exclude = (Direction::None, Duration::ZERO);

    if tags.is("highway", "footway") && !tags.is_any("bicycle", vec!["yes", "designated"]) {
        return exclude;
    }

    // Exclude dedicated sidewalks; they're almost always parallel to a road that should be
    // edited instead
    if tags.is("footway", "sidewalk") {
        return exclude;
    }
    // These don't have the potential to become part of a network
    if tags.is("highway", "steps") {
        return exclude;
    }

    // 10mph
    let speed = 4.4704;
    let cost = Duration::from_secs_f64(linestring.length::<Euclidean>() / speed);
    (Direction::Both, cost)
}

/// This is used for the directness metric. It looks at one-ways and speed limit, but not turn
/// restrictions.
pub fn car_profile(tags: &Tags, linestring: &LineString) -> (Direction, Duration) {
    let exclude = (Direction::None, Duration::ZERO);

    if tags.is_any(
        "highway",
        vec![
            "cycleway",
            "pedestrian",
            "footway",
            "path",
            "steps",
            "construction",
        ],
    ) {
        return exclude;
    }

    // TODO Handle private access, modal filters, etc

    let dir = if tags.is("oneway", "yes") {
        Direction::Forwards
    } else {
        Direction::Both
    };
    // mph to m/s
    let speed = (get_speed_mph(tags) as f64) / 0.44704;
    let cost = Duration::from_secs_f64(linestring.length::<Euclidean>() / speed);
    (dir, cost)
}

impl MapModel {
    pub fn classify_existing_network(&self) -> Result<String> {
        let mut features = Vec::new();
        for road in &self.graph.roads {
            if let Some(infra_type) = classify(&road.osm_tags) {
                let mut f = self.graph.mercator.to_wgs84_gj(&road.linestring);
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

    // Test if ways should be included in the model at all
    #[test]
    fn test_bicycle_profile() {
        let mut ok = true;
        for (input, should_include) in [
            // https://www.openstreetmap.org/way/588483433
            (vec!["highway=footway"], false),
            (vec!["highway=footway", "footway=sidewalk"], false),
            (vec!["highway=footway", "bicycle=designated"], true),
            (vec!["highway=footway", "bicycle=yes"], true),
            (vec!["highway=steps"], false),
        ] {
            let do_include =
                bicycle_profile(&tags(&input), &LineString::new(Vec::new())).0 != Direction::None;
            if should_include && !do_include {
                println!("For {input:?}, we should include it, but don't\n");
                ok = false;
            } else if !should_include && do_include {
                println!("For {input:?}, we shouldn't include it, but do\n");
                ok = false;
            }
        }

        if !ok {
            panic!("Some cases failed");
        }
    }

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
            let actual = classify(&tags(&input));
            if actual != expected {
                println!("For {input:?}, expected {expected:?} but got {actual:?}\n");
                ok = false;
            }
        }

        if !ok {
            panic!("Some cases failed");
        }
    }

    // TODO Upstream as a test utility
    fn tags(input: &Vec<&'static str>) -> Tags {
        let mut tags = Tags::empty();
        for kv in input {
            let parts = kv.split("=").collect::<Vec<_>>();
            tags.insert(parts[0], parts[1]);
        }
        tags
    }
}
