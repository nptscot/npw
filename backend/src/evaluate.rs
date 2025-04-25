use anyhow::Result;
use geo::{Coord, Distance, Euclidean, Length};
use geojson::FeatureCollection;
use graph::PathStep;
use serde::Serialize;

use crate::{
    routes::gradient_group, utils::into_object_value, InfraType, LevelOfService, MapModel,
};

pub enum Breakdown {
    None,
    LevelOfService,
    InfraType,
    Gradient,
}

impl MapModel {
    pub fn evaluate_route(&self, pt1: Coord, pt2: Coord, breakdown: Breakdown) -> Result<String> {
        assert!(self.quiet_router_ok);

        let profile = self.graph.profile_names["bicycle_direct"];
        let start = self.graph.snap_to_road(pt1, profile);
        let end = self.graph.snap_to_road(pt2, profile);
        let route = self.graph.routers[profile.0].route(&self.graph, start, end)?;
        let full_route_linestring = route.linestring(&self.graph);

        let mut directions = Vec::new();
        for step in &route.steps {
            if let PathStep::Road { road: id, .. } = step {
                let road = &self.graph.roads[id.0];
                directions.push(Step {
                    name: road.osm_tags.get("name").cloned(),
                    length: road.length_meters,
                    way: road.way.to_string(),
                    infra_type: self.get_infra_type(*id),
                    los: self.los[id.0],
                });
            }
        }

        let mut features = Vec::new();
        match breakdown {
            Breakdown::None => {
                features.push(self.graph.mercator.to_wgs84_gj(&full_route_linestring));
                let mut f = self.graph.mercator.to_wgs84_gj(&full_route_linestring);
                f.set_property("kind", "bicycle_direct");
                features.push(f);
            }
            Breakdown::LevelOfService => {
                for (linestring, los) in route.split_linestrings(&self.graph, |r| self.los[r.0]) {
                    let mut f = self.graph.mercator.to_wgs84_gj(&linestring);
                    f.set_property("kind", "bicycle_direct");
                    f.set_property("los", serde_json::to_value(los)?);
                    features.push(f);
                }
            }
            Breakdown::InfraType => {
                for (linestring, infra_type) in
                    route.split_linestrings(&self.graph, |r| self.get_infra_type(r))
                {
                    let mut f = self.graph.mercator.to_wgs84_gj(&linestring);
                    f.set_property("kind", "bicycle_direct");
                    f.set_property("infra_type", serde_json::to_value(infra_type)?);
                    features.push(f);
                }
            }
            Breakdown::Gradient => {
                for (linestring, gradient) in
                    route.split_linestrings(&self.graph, |r| gradient_group(self.gradients[r.0]))
                {
                    let mut f = self.graph.mercator.to_wgs84_gj(&linestring);
                    f.set_property("kind", "bicycle_direct");
                    f.set_property("gradient_group", gradient);
                    features.push(f);
                }
            }
        }

        let car_profile = self.graph.profile_names["car"];
        let car_start = self.graph.snap_to_road(pt1, car_profile);
        let car_end = self.graph.snap_to_road(pt2, car_profile);
        let car_route = self.graph.routers[car_profile.0].route(&self.graph, car_start, car_end)?;
        let car_linestring = car_route.linestring(&self.graph);
        {
            let mut f = self.graph.mercator.to_wgs84_gj(&car_linestring);
            f.set_property("kind", "car");
            features.push(f);
        }

        let quiet_bike_profile = self.graph.profile_names["bicycle_quiet"];
        let quiet_bike_route =
            self.graph.routers[quiet_bike_profile.0].route(&self.graph, start, end)?;
        let quiet_bike_linestring = quiet_bike_route.linestring(&self.graph);
        {
            let mut f = self.graph.mercator.to_wgs84_gj(&quiet_bike_linestring);
            f.set_property("kind", "quiet_bike");
            features.push(f);
        }

        let straight_line_length = Euclidean.distance(
            full_route_linestring.points().next().unwrap(),
            full_route_linestring.points().last().unwrap(),
        );

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: Some(into_object_value(serde_json::json!({
                "car_length": Euclidean.length(&car_linestring),
                "direct_bike_length": Euclidean.length(&full_route_linestring),
                "quiet_bike_length": Euclidean.length(&quiet_bike_linestring),
                "straight_line_length": straight_line_length,
                "directions": directions,
            }))),
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
