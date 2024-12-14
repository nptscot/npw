use anyhow::{bail, Result};
use gdal::{vector::LayerAccess, Dataset};
use geo::{Distance, Euclidean, Geometry, Length, LineString};
use graph::{Graph, Timer};
use rstar::{primitives::GeomWithData, RTree, RTreeObject};
use serde::Serialize;

use backend::Tier;

// The output is per road. If the road is part of the core network, what tier is it?
pub fn read_core_network(
    path: &str,
    graph: &Graph,
    timer: &mut Timer,
) -> Result<Vec<Option<Tier>>> {
    // Read all relevant lines and make an RTree
    timer.step("read core network");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut segments = Vec::new();
    for input in layer.features() {
        let Some(function) = input.field_as_string_by_name("road_function")? else {
            bail!("Missing road_function");
        };
        let tier = match function.as_str() {
            "Primary" => Tier::Primary,
            "Secondary" => Tier::Secondary,
            "Local Access" => Tier::LocalAccess,
            x => bail!("Unknown road_function {x}"),
        };

        let geo = input.geometry().unwrap().to_geo()?;
        match geo {
            Geometry::LineString(mut ls) => {
                graph.mercator.to_mercator_in_place(&mut ls);
                segments.push(GeomWithData::new(ls, tier));
            }
            Geometry::MultiLineString(mls) => {
                for mut ls in mls {
                    graph.mercator.to_mercator_in_place(&mut ls);
                    segments.push(GeomWithData::new(ls, tier));
                }
            }
            _ => bail!("read_core_network found something besides a LS or MLS"),
        }
    }
    let rtree = RTree::bulk_load(segments);

    // Multiple roads might match to the same segment -- dual carriageways, for example. For every
    // road, see if there's any core network segment close enough to it. Note this assumes the core
    // network is split into small segments!
    timer.step("match roads to core network segments");
    let mut output = Vec::new();
    for (idx, road) in graph.roads.iter().enumerate() {
        // TODO Plus a buffer?
        let candidates = rtree
            .locate_in_envelope_intersecting(&road.linestring.envelope())
            .collect::<Vec<_>>();
        // TODO If there are multiple hits, pick the best to get the right tier
        let best_hit = candidates
            .iter()
            .find(|obj| cn_road_geometry_similar(&road.linestring, obj.geom()))
            .map(|obj| obj.data);

        // Enable for debugging
        if false && best_hit.is_none() && !candidates.is_empty() {
            let mut out = geojson::FeatureWriter::from_writer(std::fs::File::create(format!(
                "debug{idx}.geojson"
            ))?);
            let mut f = graph.mercator.to_wgs84_gj(&road.linestring);
            f.set_property("kind", "road");
            out.write_feature(&f)?;

            for obj in candidates {
                let cmp = CompareLineStrings::new(&road.linestring, obj.geom());
                let mut f = graph.mercator.to_wgs84_gj(obj.geom());
                f.properties = Some(serde_json::to_value(&cmp)?.as_object().unwrap().clone());
                out.write_feature(&f)?;
            }
        }

        output.push(best_hit);
    }
    Ok(output)
}

#[derive(Serialize)]
struct CompareLineStrings {
    angle_main: f64,
    angle_candidate: f64,
    angle_diff: f64,

    length_main: f64,
    length_candidate: f64,
    length_ratio: f64,

    dist1: f64,
    dist2: f64,
}

impl CompareLineStrings {
    fn new(main: &LineString, candidate: &LineString) -> Self {
        let angle_main = angle_ls(main);
        let angle_candidate = angle_ls(candidate);
        let length_main = main.length::<Euclidean>();
        let length_candidate = candidate.length::<Euclidean>();
        let (dist1, dist2) = endpoint_distances(main, candidate);

        Self {
            angle_main,
            angle_candidate,
            angle_diff: (angle_main - angle_candidate).abs(),
            length_main,
            length_candidate,
            // Always >= 1
            length_ratio: if length_main >= length_candidate {
                length_main / length_candidate
            } else {
                length_candidate / length_main
            },
            dist1,
            dist2,
        }
    }
}

fn cn_road_geometry_similar(ls1: &LineString, ls2: &LineString) -> bool {
    // TODO Ignore distance between endpoints for now
    let cmp = CompareLineStrings::new(ls1, ls2);
    cmp.angle_diff <= 10.0 && cmp.length_ratio <= 1.1
}

// Angle in degrees from first to last point. Ignores the "direction" of the line; returns [0,
// 180].
fn angle_ls(ls: &LineString) -> f64 {
    let pt1 = ls.coords().next().unwrap();
    let pt2 = ls.coords().last().unwrap();
    let a1 = (pt2.y - pt1.y).atan2(pt2.x - pt1.x).to_degrees();
    // Normalize to [0, 360]
    let a2 = if a1 < 0.0 { a1 + 360.0 } else { a1 };
    // Ignore direction
    if a2 > 180.0 {
        a2 - 180.0
    } else {
        a2
    }
}

// Distance in meters between start points and between end points
fn endpoint_distances(ls1: &LineString, ls2: &LineString) -> (f64, f64) {
    let dist1 = Euclidean::distance(*ls1.coords().next().unwrap(), *ls2.coords().next().unwrap());
    let dist2 = Euclidean::distance(*ls1.coords().last().unwrap(), *ls2.coords().last().unwrap());
    (dist1, dist2)
}
