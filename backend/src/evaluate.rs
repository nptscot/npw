use anyhow::Result;
use geo::{Coord, Euclidean, Length, LineString};
use geojson::FeatureCollection;
use graph::PathStep;
use serde::Serialize;

use crate::{InfraType, LevelOfService, MapModel};

impl MapModel {
    pub fn evaluate_route(&self, pt1: Coord, pt2: Coord) -> Result<String> {
        let profile = self.graph.profile_names["bicycle"];
        let start = self.graph.snap_to_road(pt1, profile);
        let end = self.graph.snap_to_road(pt2, profile);
        let route = self.graph.routers[profile.0].route(&self.graph, start, end)?;
        let route_linestring = route.linestring(&self.graph);

        let mut directions = Vec::new();
        for step in route.steps {
            if let PathStep::Road { road: id, .. } = step {
                let road = &self.graph.roads[id.0];
                directions.push(Step {
                    name: road.osm_tags.get("name").cloned(),
                    length: road.length_meters,
                    way: road.way.to_string(),
                    infra_type: self.get_infra_type(id),
                    los: self.los[id.0],
                });
            }
        }

        let direct_line = LineString::new(vec![
            self.graph.intersections[start.intersection.0].point.into(),
            self.graph.intersections[end.intersection.0].point.into(),
        ]);
        Ok(serde_json::to_string(&FeatureCollection {
            features: vec![self.graph.mercator.to_wgs84_gj(&route_linestring)],
            bbox: None,
            foreign_members: Some(
                serde_json::json!({
                    "direct_length": direct_line.length::<Euclidean>(),
                    "route_length": route_linestring.length::<Euclidean>(),
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
    los: LevelOfService,
}
