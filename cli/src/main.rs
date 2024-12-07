use std::collections::HashMap;
use std::fs::File;
use std::io::BufWriter;

use anyhow::{bail, Result};
use clap::Parser;
use gdal::{vector::LayerAccess, Dataset};
use geo::{Distance, Euclidean, Geometry, Length, LineString, MultiPolygon};
use graph::{Graph, Timer};
use log::info;
use rstar::{primitives::GeomWithData, RTree, RTreeObject};
use serde::{Deserialize, Serialize};

use backend::MapModel;

#[derive(Parser)]
struct Args {
    /// Path to a .osm.pbf or .xml file to convert
    #[arg(long)]
    input: String,

    /// Path to GeoJSON file with the boundary to clip the input to
    #[arg(long)]
    boundary: String,

    /// Output file to write
    #[arg(long)]
    output: String,
}

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args = Args::parse();

    let mut timer = Timer::new("build model", None);
    let osm_bytes = std::fs::read(&args.input)?;
    let boundary_gj = std::fs::read_to_string(&args.boundary)?;
    let model = create(&osm_bytes, &boundary_gj, &mut timer)?;

    timer.step("writing");
    let writer = BufWriter::new(File::create(&args.output)?);
    bincode::serialize_into(writer, &model)?;

    timer.done();
    Ok(())
}

fn create(input_bytes: &[u8], boundary_gj: &str, timer: &mut Timer) -> Result<MapModel> {
    let graph = Graph::new(
        input_bytes,
        &mut utils::osm2graph::NullReader,
        vec![
            (
                "bicycle".to_string(),
                Box::new(backend::existing::bicycle_profile),
            ),
            ("car".to_string(), Box::new(backend::existing::car_profile)),
        ],
        timer,
    )?;
    let boundary_wgs84 = read_multipolygon(boundary_gj)?;

    timer.step("loading OD zones");
    let od_zones = backend::od::Zone::parse_zones(
        std::fs::read_to_string("../data_prep/tmp/zones.geojson")?,
        &boundary_wgs84,
        &graph.mercator,
    )?;

    timer.step("loading desire lines");
    let desire_lines = read_desire_lines_csv("../data_prep/tmp/od.csv", &od_zones)?;

    timer.step("loading schools");
    let schools = backend::places::School::from_gj(
        &std::fs::read_to_string("../data_prep/tmp/schools.geojson")?,
        &boundary_wgs84,
        &graph,
    )?;

    timer.step("loading GPs/hospitals");
    let gp_hospitals = backend::places::GPHospital::from_gj(
        &std::fs::read_to_string("../data_prep/tmp/gp_practices.geojson")?,
        &std::fs::read_to_string("../data_prep/tmp/hospitals.geojson")?,
        &boundary_wgs84,
        &graph,
    )?;

    timer.step("loading town centres");
    let town_centres = backend::places::TownCentre::from_gj(
        &std::fs::read_to_string("../data_prep/tmp/town_centres.geojson")?,
        &boundary_wgs84,
        &graph,
    )?;

    timer.step("loading data zones");
    let data_zones = backend::places::DataZone::from_gj(
        &std::fs::read_to_string("../data_prep/tmp/population.geojson")?,
        &boundary_wgs84,
        &graph,
    )?;

    let traffic_volumes = read_traffic_volumes("../data_prep/tmp/traffic.gpkg", &graph, timer)?;

    let core_network = read_core_network("../data_prep/tmp/core_network.gpkg", &graph, timer)?;

    let precalculated_flows =
        read_precalculated_flows("../data_prep/tmp/combined_network.gpkg", &graph, timer)?;

    Ok(MapModel::create(
        graph,
        boundary_wgs84,
        od_zones,
        desire_lines,
        schools,
        gp_hospitals,
        town_centres,
        data_zones,
        traffic_volumes,
        core_network,
        precalculated_flows,
    ))
}

fn read_multipolygon(gj_string: &str) -> Result<MultiPolygon> {
    let gj: geojson::Feature = gj_string.parse()?;
    if matches!(
        gj.geometry.as_ref().unwrap().value,
        geojson::Value::Polygon(_)
    ) {
        Ok(MultiPolygon(vec![gj.try_into()?]))
    } else {
        Ok(gj.try_into()?)
    }
}

fn read_desire_lines_csv(
    path: &str,
    zones: &HashMap<String, backend::od::Zone>,
) -> Result<Vec<(String, String, usize)>> {
    let mut out = Vec::new();
    for rec in csv::Reader::from_reader(File::open(path)?).deserialize() {
        let row: DesireLineRow = rec?;
        if zones.contains_key(&row.geo_code1) && zones.contains_key(&row.geo_code2) {
            out.push((row.geo_code1, row.geo_code2, row.all));
        }
    }
    Ok(out)
}

#[derive(Deserialize)]
struct DesireLineRow {
    geo_code1: String,
    geo_code2: String,
    all: usize,
}

fn read_traffic_volumes(path: &str, graph: &Graph, timer: &mut Timer) -> Result<Vec<usize>> {
    // Read all relevant lines and make an RTree
    timer.step("read traffic volumes");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut traffic_links = Vec::new();
    for input in layer.features() {
        let mut geom: LineString = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);
        let Some(flows) = input.field_as_integer_by_name("pred_flows")? else {
            // TODO Why missing?
            continue;
        };
        traffic_links.push(GeomWithData::new(geom, flows as usize));
    }
    if traffic_links.is_empty() {
        bail!("No traffic volumes detected; input is broken");
    }
    info!("{} links have pred_flows", traffic_links.len());
    let rtree = RTree::bulk_load(traffic_links);

    // Multiple roads might match to the same traffic link -- dual carriageways, for example.
    // Insist on finding a match for every road.
    //
    // TODO Check results carefully. npt build.R does more work.
    timer.step("match roads to traffic volumes");
    let mut output = Vec::new();
    for road in &graph.roads {
        // The source data excludes these, so we often end up matching nearby main roads with high
        // volume. Force them to 0, so they get a good LoS and don't appear as severances.
        if road.osm_tags.is("highway", "service") {
            output.push(0);
            continue;
        }

        // TODO May need a buffer here too
        if let Some(link) = rtree
            .locate_in_envelope_intersecting(&road.linestring.envelope())
            .min_by_key(|link| compare_road_geometry(&road.linestring, link.geom()))
        {
            //info!("{} got flow {} with score {} cm", road.way, link.data, compare_road_geometry(&road.linestring, link.geom()));
            output.push(link.data);
        } else {
            //info!("{} didn't match a traffic volume link", road.way);
            output.push(0);
        }
    }
    Ok(output)
}

// The output is whether each road is part of the core network or not
fn read_core_network(path: &str, graph: &Graph, timer: &mut Timer) -> Result<Vec<bool>> {
    // Read all relevant lines and make an RTree
    timer.step("read core network");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut segments = Vec::new();
    for input in layer.features() {
        let geo = input.geometry().unwrap().to_geo()?;
        match geo {
            Geometry::LineString(mut ls) => {
                graph.mercator.to_mercator_in_place(&mut ls);
                segments.push(ls);
            }
            Geometry::MultiLineString(mls) => {
                for mut ls in mls {
                    graph.mercator.to_mercator_in_place(&mut ls);
                    segments.push(ls);
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
        let any_hits = candidates
            .iter()
            .any(|geom| cn_road_geometry_similar(&road.linestring, geom));

        // Enable for debugging
        if false && !any_hits && !candidates.is_empty() {
            let mut out = geojson::FeatureWriter::from_writer(std::fs::File::create(format!(
                "debug{idx}.geojson"
            ))?);
            let mut f = graph.mercator.to_wgs84_gj(&road.linestring);
            f.set_property("kind", "road");
            out.write_feature(&f)?;

            for x in candidates {
                let cmp = CompareLineStrings::new(&road.linestring, x);
                let mut f = graph.mercator.to_wgs84_gj(x);
                f.properties = Some(serde_json::to_value(&cmp)?.as_object().unwrap().clone());
                out.write_feature(&f)?;
            }
        }

        output.push(any_hits);
    }
    Ok(output)
}

// The output is the Go Dutch totals for all purpose
fn read_precalculated_flows(path: &str, graph: &Graph, timer: &mut Timer) -> Result<Vec<usize>> {
    // Read all relevant lines and make an RTree
    timer.step("read precalculated flows");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut links = Vec::new();
    for input in layer.features() {
        let mut geom: LineString = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);
        // TODO Or quietest?
        let Some(flow) = input.field_as_integer_by_name("all_fastest_bicycle_go_dutch")? else {
            bail!("combined_network is missing all_fastest_bicycle_go_dutch");
        };
        links.push(GeomWithData::new(geom, flow as usize));
    }
    let rtree = RTree::bulk_load(links);

    // Multiple roads might match to the same link -- dual carriageways, for example.
    // Insist on finding a match for every road.
    timer.step("match roads to precalculated flows");
    let mut output = Vec::new();
    for road in &graph.roads {
        // TODO Skip service roads or similar? Check what's in the NPT rnet

        if let Some(link) = rtree
            .locate_in_envelope_intersecting(&road.linestring.envelope())
            .min_by_key(|link| compare_road_geometry(&road.linestring, link.geom()))
        {
            output.push(link.data);
        } else {
            output.push(0);
        }
    }
    Ok(output)
}

// Just sum distance between endpoints
// TODO When the volume links are much longer than OSM, or vice versa, how well does this work?
fn compare_road_geometry(ls1: &LineString, ls2: &LineString) -> usize {
    let dist1 = Euclidean::distance(*ls1.coords().next().unwrap(), *ls2.coords().next().unwrap());
    let dist2 = Euclidean::distance(*ls1.coords().last().unwrap(), *ls2.coords().last().unwrap());
    // cm precision
    ((dist1 + dist2) * 100.0) as usize
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
