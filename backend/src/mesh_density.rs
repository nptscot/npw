use anyhow::Result;
use geo::{
    BooleanOps, BoundingRect, Coord, Euclidean, Intersects, Length, LineString, MultiLineString,
    Rect,
};
use geojson::FeatureCollection;
use utils::Grid;

use crate::MapModel;

impl MapModel {
    pub fn calculate_grid_mesh_density(
        &self,
        resolution: f64,
        x_offset: f64,
        y_offset: f64,
    ) -> Result<Vec<u8>> {
        // Make a 2D grid covering the entire area. Each tile counts (total length of routes built
        // inside, total length of all roads inside).
        // TODO We can stop counting the total length of roads inside?
        // TODO We don't actually use the grid structure at all
        let mut grid: Grid<(f64, f64)> = Grid::new(
            (self.graph.mercator.width / resolution).ceil() as usize,
            (self.graph.mercator.height / resolution).ceil() as usize,
            (0.0, 0.0),
        );

        // Clip every drawn linestring to the grid
        for route in self.routes.values() {
            let ls = self.graph.mercator.to_mercator(&route.linestring_wgs84);

            visit_grid(
                &mut grid,
                resolution,
                x_offset,
                y_offset,
                ls,
                |pair, clipped| {
                    pair.0 += Euclidean.length(&clipped);
                },
            );
        }

        // And every road
        for road in &self.graph.roads {
            visit_grid(
                &mut grid,
                resolution,
                x_offset,
                y_offset,
                road.linestring.clone(),
                |pair, clipped| {
                    pair.1 += Euclidean.length(&clipped);
                },
            );
        }

        let mut features = Vec::new();
        for x in 0..grid.width {
            for y in 0..grid.height {
                let (routes, total) = grid.data[grid.idx(x, y)];
                if total == 0.0 {
                    continue;
                }

                let square = Rect::new(
                    Coord {
                        x: ((x as f64) * resolution) - x_offset,
                        y: ((y as f64) * resolution) - y_offset,
                    },
                    Coord {
                        x: (((x + 1) as f64) * resolution) - x_offset,
                        y: (((y + 1) as f64) * resolution) - y_offset,
                    },
                )
                .to_polygon();

                // Also check if any settlement intersects this
                if self
                    .settlements
                    .iter()
                    .all(|s| !s.polygon.intersects(&square))
                {
                    continue;
                }

                let mut f = self.graph.mercator.to_wgs84_gj(&square);
                f.set_property("routes", routes);
                f.set_property("total", total);
                features.push(f);
            }
        }
        Ok(serde_json::to_vec(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
    }
}

// TODO Upstream to grid
fn visit_grid<F: Fn(&mut (f64, f64), MultiLineString)>(
    grid: &mut Grid<(f64, f64)>,
    resolution: f64,
    x_offset: f64,
    y_offset: f64,
    input: LineString,
    update: F,
) {
    let mls = MultiLineString(vec![input]);

    let bbox: Rect = mls.bounding_rect().unwrap().into();
    // TODO Grid should have more helpers
    // Roads may go out of bounds. Negatives round to 0.
    let x1 = (bbox.min().x / resolution).round().max(0.0) as usize;
    let x2 = (bbox.max().x / resolution).round().max(0.0) as usize;
    let y1 = (bbox.min().y / resolution).round().max(0.0) as usize;
    let y2 = (bbox.max().y / resolution).round().max(0.0) as usize;

    for x in x1..=x2 {
        for y in y1..=y2 {
            // Some roads go out-of-bounds
            if x1 >= grid.width || x2 >= grid.width || y1 >= grid.height || y2 >= grid.height {
                continue;
            }

            let square = Rect::new(
                Coord {
                    x: ((x as f64) * resolution) - x_offset,
                    y: ((y as f64) * resolution) - y_offset,
                },
                Coord {
                    x: (((x + 1) as f64) * resolution) - x_offset,
                    y: (((y + 1) as f64) * resolution) - y_offset,
                },
            )
            .to_polygon();

            let invert = false;
            let clipped = square.clip(&mls, invert);
            let idx = grid.idx(x, y);
            update(&mut grid.data[idx], clipped);
        }
    }
}
