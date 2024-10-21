use anyhow::Result;
use geo::{Coord, LineString, Polygon};
use geojson::{Feature, FeatureCollection, Geometry};
use i_float::f64_point::F64Point;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::f64::string::F64StringOverlay;
use i_overlay::string::rule::StringRule;

use crate::MapModel;

impl MapModel {
    pub fn calculate_mesh_density(&self) -> Result<String> {
        let linestrings = self.get_mesh_density_sources();
        let boundary = self.graph.mercator.to_mercator(&self.boundary.0[0]);
        let polygons = split_polygon(boundary, linestrings);

        let mut features = Vec::new();
        for ls in polygons {
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

    fn get_mesh_density_sources(&self) -> Vec<LineString> {
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
        linestrings
    }
}

fn split_polygon(polygon: Polygon, linestrings: Vec<LineString>) -> Vec<Polygon> {
    let mut overlay = F64StringOverlay::new();
    overlay.add_shape_path(polygon.exterior().coords().map(to_pt).collect());
    for ls in linestrings {
        overlay.add_string_lines(
            ls.lines()
                .map(|l| [to_pt(&l.start), to_pt(&l.end)])
                .collect(),
        );
    }

    let graph = overlay.into_graph(FillRule::NonZero);
    let shapes = graph.extract_shapes(StringRule::Slice);

    shapes.into_iter().map(to_geo_polygon).collect()
}

fn to_pt(pt: &Coord) -> F64Point {
    F64Point::new(pt.x, pt.y)
}

fn to_geo_polygon(rings: Vec<Vec<F64Point>>) -> Polygon {
    let mut interiors: Vec<LineString> = rings.into_iter().map(to_geo_linestring).collect();
    let exterior = interiors.remove(0);
    Polygon::new(exterior, interiors)
}

fn to_geo_linestring(pts: Vec<F64Point>) -> LineString {
    LineString(
        pts.into_iter()
            .map(|pt| Coord { x: pt.x, y: pt.y })
            .collect(),
    )
}
