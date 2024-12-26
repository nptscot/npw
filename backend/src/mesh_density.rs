use anyhow::Result;
use geo::{Area, Contains, Coord, LineString, Polygon, Rect};
use geojson::FeatureCollection;
use i_float::f64_point::F64Point;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::f64::string::F64StringOverlay;
use i_overlay::string::rule::StringRule;
use utils::Grid;

use crate::MapModel;

impl MapModel {
    pub fn calculate_area_mesh_density(&self) -> Result<String> {
        let linestrings = self.get_mesh_density_sources();
        let boundary = self.graph.mercator.to_mercator(&self.boundary_wgs84.0[0]);
        let polygons = split_polygon(boundary, linestrings);

        let mut with_area = polygons
            .into_iter()
            .map(|polygon| {
                // Convert from m^2 to km^2. Use unsigned area to ignore polygon orientation.
                let area = polygon.unsigned_area() / 1_000_000.0;
                (polygon, area)
            })
            .collect::<Vec<_>>();
        // Put smallest areas on top, for better z-ordering in the frontend. In practice, the areas
        // overlap.
        with_area.sort_by_key(|pair| (pair.1 * 1000.0) as usize);
        with_area.reverse();

        let mut features = Vec::new();
        for (polygon, area) in with_area {
            let mut f = self.graph.mercator.to_wgs84_gj(&polygon);
            f.set_property("area", area);
            features.push(f);
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
    }

    fn get_mesh_density_sources(&self) -> Vec<LineString> {
        // Anything with infrastructure on it
        let mut linestrings = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            if self.infra_types[idx].is_some() {
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

impl MapModel {
    pub fn calculate_grid_mesh_density(&self) -> Result<String> {
        // TODO Depends on urban/rural
        let resolution = 200.0;

        // Make a 2D grid covering the entire area. Each tile counts the total length of routes built inside.
        let grid: Grid<f64> = Grid::new(
            (self.graph.mercator.width / resolution).ceil() as usize,
            (self.graph.mercator.height / resolution).ceil() as usize,
            0.0,
        );

        let boundary = self.graph.mercator.to_mercator(&self.boundary_wgs84.0[0]);
        let mut features = Vec::new();
        for x in 0..grid.width {
            for y in 0..grid.height {
                let midpt = Coord {
                    x: ((x as f64) + 0.5) * resolution,
                    y: ((y as f64) + 0.5) * resolution,
                };
                if !boundary.contains(&midpt) {
                    continue;
                }

                let square = Rect::new(
                    Coord {
                        x: (x as f64) * resolution,
                        y: (y as f64) * resolution,
                    },
                    Coord {
                        x: ((x + 1) as f64) * resolution,
                        y: ((y + 1) as f64) * resolution,
                    },
                )
                .to_polygon();
                let mut f = self.graph.mercator.to_wgs84_gj(&square);
                f.set_property("length", grid.data[grid.idx(x, y)]);
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
