use anyhow::Result;
use cavalier_contours::polyline::{
    BooleanOp, PlineCreation, PlineOffsetOptions, PlineSource, PlineVertex, Polyline,
};
use geo::{BoundingRect, Coord, GeometryCollection, LineString, Polygon, Rect};
use geojson::{Feature, FeatureCollection, Geometry};

use crate::MapModel;

impl MapModel {
    pub fn calculate_mesh_density(&self) -> Result<String> {
        let (linestrings, bbox) = self.get_mesh_density_sources();
        let linestrings = crate::join_lines::collapse_degree_2(linestrings);
        //let linestrings = crate::join_lines::join_linestrings(linestrings);

        let mut features = Vec::new();
        /*features.push(Feature::from(Geometry::from(
            &self.graph.mercator.to_wgs84(&bbox),
        )));*/
        for ls in linestrings {
            features.push(Feature::from(Geometry::from(
                &self.graph.mercator.to_wgs84(&ls),
            )));
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
    }

    fn get_mesh_density_sources(&self) -> (Vec<LineString>, Polygon) {
        // Find all main roads (later this'll be the drawn cycle network, but most sketches aren't
        // likely to be complete enough yet)
        let mut linestrings = Vec::new();
        for road in &self.graph.roads {
            if road.osm_tags.is_any(
                "highway",
                vec![
                    "motorway",
                    "motorway_link",
                    "primary",
                    "primary_link",
                    "secondary",
                    "secondary_link",
                ],
            ) {
                linestrings.push(road.linestring.clone());
            }
        }

        // Ultimately we'll divide the study area boundary, but since it's built from a convex hull
        // of geometry right now, roads aren't likely to cross it. Use a slightly shrunken bbox
        // around these linestrings, just to focus on splitting that polygon.
        let gc = GeometryCollection::from(linestrings.clone());
        let bbox = gc.bounding_rect().unwrap();
        let tighter_bbox = Rect::new(
            Coord {
                x: bbox.min().x + 0.2 * bbox.width(),
                y: bbox.min().y + 0.2 * bbox.height(),
            },
            Coord {
                x: bbox.max().x - 0.2 * bbox.width(),
                y: bbox.max().y - 0.2 * bbox.height(),
            },
        )
        .to_polygon();

        (linestrings, tighter_bbox)
    }
}

fn linestring_to_pline(linestring: &LineString) -> Polyline {
    let is_closed = false;
    Polyline::from_iter(
        linestring
            .0
            .iter()
            .map(|pt| PlineVertex::new(pt.x, pt.y, 0.0)),
        is_closed,
    )
}

fn pline_to_linestring(pline: &Polyline) -> LineString {
    LineString::new(
        pline
            .vertex_data
            .iter()
            .map(|v| Coord { x: v.x, y: v.y })
            .collect(),
    )
}
