use std::time::Duration;

use anyhow::Result;
use geo::{Euclidean, Length, LineString};
use geojson::FeatureCollection;
use graph::Direction;
use utils::Tags;

use crate::{level_of_service::get_speed_mph, InfraType, MapModel};

/// All of the OSM highway types used anywhere. This forces exhaustive matching of all cases.
pub enum Highway {
    Motorway,
    Trunk,
    Primary,
    Secondary,
    Tertiary,
    Residential,
    Service,
    Unclassified,

    Footway,
    Cycleway,
    Pedestrian,
    Path,
}

impl Highway {
    // This is somewhat based on
    // https://github.com/nptscot/osmactive/blob/b08d91b310187c6b344d3682e040e47ce2519be1/R/osmactive.R#L133-L316,
    // but the purpose is different -- unless a road can't be modified, then it still belongs in
    // the graph.
    pub fn classify(tags: &Tags) -> Option<Self> {
        match tags.get("highway")?.as_str() {
            "motorway" | "motorway_link" => Some(Highway::Motorway),
            "trunk" | "trunk_link" => Some(Highway::Trunk),
            "primary" | "primary_link" => Some(Highway::Primary),
            "secondary" | "secondary_link" => Some(Highway::Secondary),
            "tertiary" | "tertiary_link" => Some(Highway::Tertiary),
            "residential" => Some(Highway::Residential),
            "service" => Some(Highway::Service),
            "unclassified" => Some(Highway::Unclassified),
            "cycleway" => Some(Highway::Cycleway),
            "pedestrian" => Some(Highway::Pedestrian),
            "path" => Some(Highway::Path),
            "footway" => {
                // Exclude dedicated sidewalks; they're almost always parallel to a road that
                // should be edited instead
                if tags.is_any("bicycle", vec!["yes", "designated"])
                    && !tags.is("footway", "sidewalk")
                {
                    Some(Highway::Footway)
                } else {
                    None
                }
            }
            // TODO Make sure we got all cases; print stuff. (steps, construction...)
            _ => None,
        }
    }
}

/// This determines what's in the graph. The cost function is just based on distance.
pub fn bicycle_profile(tags: &Tags, linestring: &LineString) -> (Direction, Duration) {
    let exclude = (Direction::None, Duration::ZERO);

    if Highway::classify(tags).is_none() {
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

    if let Some(hwy) = Highway::classify(tags) {
        if matches!(
            hwy,
            Highway::Footway | Highway::Cycleway | Highway::Pedestrian | Highway::Path
        ) {
            return exclude;
        }
    } else {
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
    match Highway::classify(tags).unwrap() {
        Highway::Motorway
        | Highway::Trunk
        | Highway::Primary
        | Highway::Secondary
        | Highway::Tertiary
        | Highway::Residential
        | Highway::Service
        | Highway::Unclassified => {
            if is_any_key(
                tags,
                vec![
                    "cycleway",
                    "cycleway:left",
                    "cycleway:right",
                    "cycleway:both",
                ],
                "track",
            ) {
                if is_wide_track(tags) {
                    return Some(InfraType::SegregatedWide);
                } else {
                    return Some(InfraType::SegregatedNarrow);
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

        Highway::Footway | Highway::Path => {
            if is_off_road(tags) {
                return Some(InfraType::OffRoad);
            }

            if tags.is("segregated", "yes") {
                // TODO Not sure
                return Some(InfraType::OffRoad);
            } else {
                return Some(InfraType::SharedFootway);
            }
        }
        Highway::Cycleway => {
            if is_off_road(tags) {
                return Some(InfraType::OffRoad);
            }

            if is_wide_track(tags) {
                Some(InfraType::SegregatedWide)
            } else {
                Some(InfraType::SegregatedNarrow)
            }
        }
        Highway::Pedestrian => {
            if is_off_road(tags) {
                return Some(InfraType::OffRoad);
            }

            None
        }
    }
}

fn is_off_road(tags: &Tags) -> bool {
    // TODO maybe regex
    if let Some(name) = tags.get("name") {
        // TODO case sensitivity?
        return ["Path", "Towpath", "Railway", "Trail"]
            .iter()
            .any(|x| name.contains(x));
    }
    false
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
