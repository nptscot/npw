use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use anime::Anime;
use anyhow::{bail, Result};
use clap::Parser;
use elevation::GeoTiffElevation;
use gdal::{vector::LayerAccess, Dataset};
use geo::{Centroid, Geometry, Intersects, LineString, MultiPolygon, Point};
use graph::{Graph, RoadID, Timer};
use log::{info, warn};
use serde::Deserialize;

use self::disconnected::remove_disconnected_components;
use backend::{Highway, MapModel, Streetspace, Tier};

mod disconnected;

#[derive(Parser)]
struct Args {
    /// Path to a .osm.pbf or .xml file to convert
    #[arg(long)]
    input: String,

    /// Path to GeoJSON file with the boundary to clip the input to
    #[arg(long)]
    boundary: String,

    /// Map model output file to write
    #[arg(long)]
    output: String,

    /// Baseline stats output file to write
    #[arg(long)]
    stats_output: String,
}

fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let args = Args::parse();

    let mut timer = Timer::new("build model", None);
    let osm_bytes = std::fs::read(&args.input)?;
    let boundary_gj = std::fs::read_to_string(&args.boundary)?;
    let study_area_name = args.output.split("/").last().unwrap().strip_suffix(".bin").unwrap();
    let model = create(study_area_name, &osm_bytes, &boundary_gj, &mut timer)?;

    timer.step("writing");
    let writer = BufWriter::new(File::create(&args.output)?);
    bincode::serialize_into(writer, &model)?;

    std::fs::write(
        &args.stats_output,
        serde_json::to_string(&model.get_baseline_stats())?,
    )?;

    timer.done();
    Ok(())
}

fn create(
    study_area_name: &str,
    input_bytes: &[u8],
    boundary_gj: &str,
    timer: &mut Timer,
) -> Result<MapModel> {
    info!("Creating MapModel for {study_area_name}");
    let graph = Graph::new(
        input_bytes,
        &mut utils::osm2graph::NullReader,
        Box::new(remove_disconnected_components),
        vec![
            (
                "bicycle_quiet".to_string(),
                Box::new(backend::existing::bicycle_profile),
            ),
            // TODO This is wasteful in a few ways. bicycle_quiet will change with edits, but
            // bicycle_direct is immutable. At least clone the Router.
            (
                "bicycle_direct".to_string(),
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

    timer.step("loading settlements");
    let settlements = backend::places::Settlement::from_gj(
        &std::fs::read_to_string("../data_prep/tmp/settlements.geojson")?,
        &boundary_wgs84,
        &graph,
    )?;

    timer.step("loading data zones");
    let data_zones = backend::places::DataZone::from_gj(
        &std::fs::read_to_string("../data_prep/tmp/population.geojson")?,
        &boundary_wgs84,
        &graph,
    )?;

    timer.step("loading greenspaces");
    let greenspaces = read_greenspaces(
        "../data_prep/tmp/greenspace.gpkg",
        "../data_prep/tmp/greenspace_access_points.gpkg",
        &boundary_wgs84,
        &graph,
    )?;

    let traffic_volumes = read_traffic_volumes("../data_prep/tmp/traffic.gpkg", &graph, timer)?;

    let core_network = read_core_network("../data_prep/tmp/core_network.gpkg", &graph, timer)?;

    let precalculated_flows =
        read_precalculated_flows("../data_prep/tmp/combined_network.gpkg", &graph, timer)?;

    let street_space = read_street_space("../data_prep/tmp/streetspace.gpkg", &graph, timer)?;

    let gradients = read_gradients("../data_prep/tmp/UK-dem-50m-4326.tif", &graph, timer)?;

    Ok(MapModel::create(
        study_area_name,
        graph,
        boundary_wgs84,
        od_zones,
        desire_lines,
        schools,
        gp_hospitals,
        town_centres,
        settlements,
        data_zones,
        greenspaces,
        traffic_volumes,
        core_network,
        precalculated_flows,
        street_space,
        gradients,
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
    // Read all relevant lines
    timer.step("read traffic volumes");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut source_geometry = Vec::new();
    let mut source_data = Vec::new();
    for input in layer.features() {
        let mut geom: LineString = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);
        let Some(flows) = input.field_as_integer_by_name("pred_flows")? else {
            // TODO Why missing?
            continue;
        };
        source_geometry.push(geom);
        source_data.push(flows as usize);
    }
    if source_geometry.is_empty() {
        bail!("No traffic volumes detected; input is broken");
    }
    info!("{} links have pred_flows", source_geometry.len());

    timer.step("match traffic volumes");
    let distance_tolerance = 15.0;
    let angle_tolerance = 10.0;
    let matches = Anime::new(
        source_geometry.into_iter(),
        graph.roads.iter().map(|r| r.linestring.clone()),
        distance_tolerance,
        angle_tolerance,
    )
    .matches
    .take()
    .unwrap();

    let mut results = Vec::new();
    for (idx, road) in graph.roads.iter().enumerate() {
        // There are issues with small dead-end service roads next to big roads and car-free roads
        // having high traffic assigned. Sometimes the problem is map matching, sometimes the
        // source dataset is wrong. Override in some cases.
        if matches!(
            Highway::classify(&road.osm_tags).unwrap(),
            Highway::Service
                | Highway::Footway
                | Highway::Cycleway
                | Highway::Pedestrian
                | Highway::Path
        ) {
            results.push(0);
            continue;
        }
        results.push(get_anime_match(&matches, &source_data, idx).unwrap_or(0));
    }
    Ok(results)
}

// The output is the Go Dutch totals for all purpose
fn read_precalculated_flows(path: &str, graph: &Graph, timer: &mut Timer) -> Result<Vec<usize>> {
    // Read all relevant lines
    timer.step("read precalculated flows");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut source_geometry = Vec::new();
    let mut source_data = Vec::new();
    for input in layer.features() {
        let mut geom: LineString = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);
        let Some(flow) = input.field_as_integer_by_name("all_fastest_bicycle_go_dutch")? else {
            bail!("combined_network is missing all_fastest_bicycle_go_dutch");
        };
        source_geometry.push(geom);
        source_data.push(flow as usize);
    }

    timer.step("match roads to precalculated flows");
    let distance_tolerance = 15.0;
    let angle_tolerance = 10.0;
    let matches = Anime::new(
        source_geometry.into_iter(),
        graph.roads.iter().map(|r| r.linestring.clone()),
        distance_tolerance,
        angle_tolerance,
    )
    .matches
    .take()
    .unwrap();

    let mut results = Vec::new();
    for idx in 0..graph.roads.len() {
        results.push(get_anime_match(&matches, &source_data, idx).unwrap_or(0));
    }
    Ok(results)
}

fn read_street_space(
    path: &str,
    graph: &Graph,
    timer: &mut Timer,
) -> Result<Vec<Option<Streetspace>>> {
    timer.step("read streetspace");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut source_geometry = Vec::new();
    let mut source_data = Vec::new();
    for input in layer.features() {
        let mut geom: LineString = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);
        let Some(available_width) = input.field_as_double_by_name("avail_edge_to_edge_width")?
        else {
            // Some are just missing
            continue;
        };
        // The absolute minimum width for a bidirectional cycletrack, from CbD guidance table 3.2.
        let segregated_fits = available_width >= 2.0;

        source_geometry.push(geom);
        source_data.push(Streetspace { segregated_fits });
    }

    timer.step("match roads to streetspace");
    let distance_tolerance = 15.0;
    let angle_tolerance = 10.0;
    let matches = Anime::new(
        source_geometry.into_iter(),
        graph.roads.iter().map(|r| r.linestring.clone()),
        distance_tolerance,
        angle_tolerance,
    )
    .matches
    .take()
    .unwrap();

    // TODO Only for big roads
    let mut results = Vec::new();
    for idx in 0..graph.roads.len() {
        results.push(get_anime_match(&matches, &source_data, idx));
    }
    Ok(results)
}

fn read_gradients(path: &str, graph: &Graph, timer: &mut Timer) -> Result<Vec<f64>> {
    timer.step("read gradients");
    let mut geotiff = GeoTiffElevation::new(BufReader::new(File::open(path)?));
    let mut gradients = Vec::new();
    for road in &graph.roads {
        // TODO This only checks the start and end point
        let pt1 = graph
            .mercator
            .pt_to_wgs84(*road.linestring.coords().next().unwrap());
        let pt2 = graph
            .mercator
            .pt_to_wgs84(*road.linestring.coords().last().unwrap());

        let Some(height1) = geotiff.get_height_for_lon_lat(pt1.x as f32, pt1.y as f32) else {
            bail!("Couldn't get height for {pt1:?}");
        };
        let Some(height2) = geotiff.get_height_for_lon_lat(pt2.x as f32, pt2.y as f32) else {
            bail!("Couldn't get height for {pt2:?}");
        };

        let slope = (height2 - height1) / (road.length_meters as f32) * 100.0;
        gradients.push(slope.into());
    }
    Ok(gradients)
}

// The output is per road. If the road is part of the core network, what tier is it?
fn read_core_network(path: &str, graph: &Graph, timer: &mut Timer) -> Result<Vec<Option<Tier>>> {
    // Read all relevant lines
    timer.step("read core network");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut source_geometry = Vec::new();
    let mut source_data = Vec::new();
    for input in layer.features() {
        let Some(function) = input.field_as_string_by_name("road_function_npt")? else {
            bail!("Missing road_function_npt");
        };
        let tier = match function.as_str() {
            "Primary" => Tier::Primary,
            "Secondary" => Tier::Secondary,
            // TODO This is supposed to be gone, but it's still here
            "Local Access" => {
                continue;
            }
            x => bail!("Unknown road_function_npt {x}"),
        };

        let geo = input.geometry().unwrap().to_geo()?;
        match geo {
            Geometry::LineString(mut ls) => {
                graph.mercator.to_mercator_in_place(&mut ls);
                source_geometry.push(ls);
                source_data.push(tier);
            }
            Geometry::MultiLineString(mls) => {
                for mut ls in mls {
                    graph.mercator.to_mercator_in_place(&mut ls);
                    source_geometry.push(ls);
                    source_data.push(tier);
                }
            }
            _ => bail!("read_core_network found something besides a LS or MLS"),
        }
    }

    timer.step("match core network");
    let distance_tolerance = 15.0;
    let angle_tolerance = 10.0;
    let matches = Anime::new(
        source_geometry.into_iter(),
        graph.roads.iter().map(|r| r.linestring.clone()),
        distance_tolerance,
        angle_tolerance,
    )
    .matches
    .take()
    .unwrap();

    let mut results = Vec::new();
    for (idx, road) in graph.roads.iter().enumerate() {
        // The core network can never be on the motorway, but in Glasgow, it's hard to distinguish
        // some of the parallel roads
        if matches!(
            Highway::classify(&road.osm_tags).unwrap(),
            Highway::Motorway
        ) {
            results.push(None);
            continue;
        }
        results.push(get_anime_match(&matches, &source_data, idx));
    }
    Ok(results)
}

fn read_greenspaces(
    main_path: &str,
    access_pts_path: &str,
    boundary_wgs84: &MultiPolygon,
    graph: &Graph,
) -> Result<Vec<backend::places::Greenspace>> {
    let mut access_points_per_site = read_access_points(access_pts_path, graph)?;
    let profile = graph.profile_names["bicycle_direct"];

    let dataset = Dataset::open(main_path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut greenspaces = Vec::new();
    for input in layer.features() {
        let Some(id) = input.field_as_string_by_name("id")? else {
            bail!("Missing id");
        };
        let name = input.field_as_string_by_name("name")?;
        let mut geom: MultiPolygon = input.geometry().unwrap().to_geo()?.try_into()?;
        if !boundary_wgs84.intersects(&geom) {
            continue;
        }
        graph.mercator.to_mercator_in_place(&mut geom);

        let access_points = access_points_per_site.remove(&id).unwrap_or_else(Vec::new);
        let roads: HashSet<RoadID> = access_points
            .iter()
            .map(|pt| graph.snap_to_road((*pt).into(), profile).road)
            .collect();

        if roads.is_empty() {
            warn!("Skipping greenspace {name:?} with no access points; it won't be reachable");
            continue;
        }

        let centroid = geom.centroid().unwrap();
        let x = centroid.x() / graph.mercator.width;
        let y = centroid.y() / graph.mercator.height;
        let sort = hilbert_2d::xy2h_continuous_f64(x, y, hilbert_2d::Variant::Hilbert);

        let centroid_wgs84 = graph.mercator.pt_to_wgs84(centroid.into());

        greenspaces.push(backend::places::Greenspace {
            polygon: geom,
            centroid_wgs84,
            name,
            access_points,
            roads,
            sort,
        });
    }
    Ok(greenspaces)
}

fn read_access_points(path: &str, graph: &Graph) -> Result<HashMap<String, Vec<Point>>> {
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    let mut pts_per_site = HashMap::new();
    for input in layer.features() {
        let Some(id) = input.field_as_string_by_name("site_id")? else {
            bail!("Missing site_id");
        };
        let mut geom: Point = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);

        pts_per_site.entry(id).or_insert_with(Vec::new).push(geom);
    }
    Ok(pts_per_site)
}

fn get_anime_match<T: Copy>(
    matches: &anime::MatchesMap,
    source_data: &Vec<T>,
    target_idx: usize,
) -> Option<T> {
    let candidates = matches.get(&target_idx)?;
    // Use the one with the most shared length
    let c = candidates
        .into_iter()
        .max_by_key(|c| (c.shared_len * 1000.0) as usize)?;
    Some(source_data[c.source_index])
}
