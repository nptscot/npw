use anyhow::Result;
use geo::{Coord, EuclideanLength, LineString};
use geojson::{Feature, FeatureCollection, Geometry};
use graph::PathStep;
use serde::Serialize;

use crate::{InfraType, MapModel};

impl MapModel {
    pub fn evaluate_route(&self, pt1: Coord, pt2: Coord) -> Result<String> {
        let infra_types = self.get_infra_types();
        let profile = self.graph.profile_names["bicycle"];
        let start = self.graph.snap_to_road(pt1, profile);
        let end = self.graph.snap_to_road(pt2, profile);
        let route = self.graph.routers[profile.0].route(&self.graph, start, end)?;
        let route_linestring = route.linestring(&self.graph);

        let mut directions = Vec::new();
        for step in route.steps {
            if let PathStep::Road { road, .. } = step {
                let road = &self.graph.roads[road.0];
                directions.push(Step {
                    name: road.osm_tags.get("name").cloned(),
                    length: road.length_meters,
                    way: road.way.to_string(),
                    infra_type: infra_types
                        .get(&road.id)
                        .cloned()
                        .unwrap_or(InfraType::MixedTraffic),
                });
            }
        }

        let direct_line = LineString::new(vec![
            self.graph.intersections[start.intersection.0].point.into(),
            self.graph.intersections[end.intersection.0].point.into(),
        ]);
        Ok(serde_json::to_string(&FeatureCollection {
            features: vec![Feature::from(Geometry::from(
                &self.graph.mercator.to_wgs84(&route_linestring),
            ))],
            bbox: None,
            foreign_members: Some(
                serde_json::json!({
                    "direct_length": direct_line.euclidean_length(),
                    "route_length": route_linestring.euclidean_length(),
                    "directions": directions,
                })
                .as_object()
                .unwrap()
                .clone(),
            ),
        })?)
    }
}

#[derive(Serialize)]
struct Step {
    name: Option<String>,
    length: f64,
    way: String,
    infra_type: InfraType,
}
