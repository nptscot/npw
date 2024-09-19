use anyhow::Result;
use geo::Coord;
use graph::Mode;

use geojson::{Feature, GeoJson, Geometry};

use crate::MapModel;

impl MapModel {
    pub fn evaluate_route(&self, pt1: Coord, pt2: Coord) -> Result<String> {
        let mode = Mode::Bicycle;
        let start = self.graph.snap_to_road(pt1, mode);
        let end = self.graph.snap_to_road(pt2, mode);

        let linestring = self.graph.router[mode]
            .route(&self.graph, start, end)?
            .linestring(&self.graph);

        let f = Feature::from(Geometry::from(&self.graph.mercator.to_wgs84(&linestring)));
        Ok(serde_json::to_string(&GeoJson::from(vec![f]))?)
    }
}
