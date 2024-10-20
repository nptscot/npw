use anyhow::Result;
use geojson::{Feature, FeatureCollection, Geometry};
use utils::Tags;

use crate::{InfraType, MapModel};

impl MapModel {
    pub fn classify_existing_network(&self) -> Result<String> {
        let mut features = Vec::new();
        for road in &self.graph.roads {
            if let Some(infra_type) = classify(&road.osm_tags) {
                let mut f = Feature::from(Geometry::from(
                    &self.graph.mercator.to_wgs84(&road.linestring),
                ));
                f.set_property("infra_type", serde_json::to_value(infra_type)?);
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
fn classify(tags: &Tags) -> Option<InfraType> {
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
