use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufWriter};

use anime::Anime;
use anyhow::{bail, Result};
use clap::Parser;
use elevation::GeoTiffElevation;
use gdal::{vector::LayerAccess, Dataset};
use geo::{
    Area, BoundingRect, Buffer, Centroid, Contains, Coord, Geometry, Intersects, LineString,
    MultiPolygon, Point, Polygon, Rect,
};
use graph::{Graph, RoadID, Timer};
use log::{info, warn};
use rstar::AABB;
use serde::Deserialize;
use utils::Tags;

use self::disconnected::remove_disconnected_components;
use backend::{Highway, MapModel, Streetspace, Tier, TrafficVolume};

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
    let study_area_name = args
        .output
        .split("/")
        .last()
        .unwrap()
        .strip_suffix(".bin")
        .unwrap();
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
        ],
        timer,
    )?;
    let boundary_wgs84 = read_multipolygon(boundary_gj)?;

    timer.step("loading data zones");
    let data_zones = backend::places::DataZone::from_gj(
        &std::fs::read_to_string("../data_prep/tmp/population.geojson")?,
        &boundary_wgs84,
        &graph,
    )?;
    let zone_ids: HashMap<String, usize> = data_zones
        .iter()
        .enumerate()
        .map(|(idx, zone)| (zone.id.clone(), idx))
        .collect();

    timer.step("loading commute desire lines");
    let commute_desire_lines =
        read_commute_desire_lines_csv("../data_prep/tmp/od_commute.csv", &zone_ids)?;
    timer.step("loading utility and school desire lines");
    let mut other_desire_lines = read_other_desire_lines_csv(
        "../data_prep/tmp/od_utility.csv",
        &zone_ids,
        &graph,
        &boundary_wgs84,
    )?;
    // TODO Combine counts for utility and school trips
    other_desire_lines.extend(read_other_desire_lines_csv(
        "../data_prep/tmp/od_school.csv",
        &zone_ids,
        &graph,
        &boundary_wgs84,
    )?);

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

    timer.step("loading railway stations");
    let railway_stations = backend::places::RailwayStation::from_gj(
        &std::fs::read_to_string("../data_prep/tmp/railways.geojson")?,
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

    let highways: Vec<_> = graph
        .roads
        .iter()
        .map(|r| Highway::classify(&r.osm_tags).unwrap())
        .collect();

    let traffic_volumes =
        read_traffic_volumes("../data_prep/tmp/clos.gpkg", &graph, &highways, timer)?;

    let speeds = graph
        .roads
        .iter()
        .enumerate()
        .map(|(idx, r)| get_speed_mph(highways[idx], &r.osm_tags))
        .collect();

    let coherent_network = read_coherent_network(
        "../data_prep/tmp/coherent_network.gpkg",
        &graph,
        &highways,
        timer,
    )?;

    let street_space = read_street_space("../data_prep/tmp/streetspace.gpkg", &graph, timer)?;

    let is_attractive = find_streets_by_greenspace(
        "../data_prep/tmp/all_greenspace.gpkg",
        &boundary_wgs84,
        &graph,
        timer,
    )?;

    let gradients = read_gradients("../data_prep/tmp/UK-dem-50m-4326.tif", &graph, timer)?;

    Ok(MapModel::create(
        graph,
        boundary_wgs84,
        commute_desire_lines,
        other_desire_lines,
        schools,
        gp_hospitals,
        railway_stations,
        greenspaces,
        town_centres,
        settlements,
        data_zones,
        highways,
        traffic_volumes,
        speeds,
        coherent_network,
        street_space,
        is_attractive,
        gradients,
        timer,
    )?)
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

fn read_commute_desire_lines_csv(
    path: &str,
    zone_ids: &HashMap<String, usize>,
) -> Result<Vec<(usize, usize, usize)>> {
    let mut out = Vec::new();
    for rec in csv::Reader::from_reader(File::open(path)?).deserialize() {
        let row: CommuteDesireLineRow = rec?;
        if let (Some(zone1), Some(zone2)) =
            (zone_ids.get(&row.geo_code1), zone_ids.get(&row.geo_code2))
        {
            out.push((*zone1, *zone2, row.count));
        }
    }
    if out.is_empty() {
        bail!("No matching commute desire lines");
    }
    Ok(out)
}

#[derive(Deserialize)]
struct CommuteDesireLineRow {
    geo_code1: String,
    geo_code2: String,
    count: usize,
}

fn read_other_desire_lines_csv(
    path: &str,
    zone_ids: &HashMap<String, usize>,
    graph: &Graph,
    boundary_wgs84: &MultiPolygon,
) -> Result<Vec<(usize, Coord, usize)>> {
    let mut out = Vec::new();
    for rec in csv::Reader::from_reader(File::open(path)?).deserialize() {
        let row: OtherDesireLineRow = rec?;
        // Around 84k rows aren't useful
        if row.count == 0 {
            continue;
        }
        if let Some(zone1) = zone_ids.get(&row.geo_code1) {
            let pt = Coord {
                x: row.destination_lon,
                y: row.destination_lat,
            };
            if boundary_wgs84.contains(&pt) {
                out.push((*zone1, graph.mercator.pt_to_mercator(pt), row.count));
            }
        }
    }
    if out.is_empty() {
        bail!("No matching utility or school desire lines");
    }
    info!("Read {} utility or school desire lines", out.len());
    Ok(out)
}

#[derive(Deserialize)]
struct OtherDesireLineRow {
    geo_code1: String,
    destination_lon: f64,
    destination_lat: f64,
    count: usize,
}

fn read_traffic_volumes(
    path: &str,
    graph: &Graph,
    highways: &Vec<Highway>,
    timer: &mut Timer,
) -> Result<Vec<TrafficVolume>> {
    // Read all relevant lines
    timer.step("read traffic volumes");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);
    let traffic_idx = layer.defn().field_index("Traffic.volume.category")?;

    let mut source_geometry = Vec::new();
    let mut source_data = Vec::new();
    for input in layer.features() {
        let Some(traffic) = input.field_as_string(traffic_idx)? else {
            // TODO Why missing?
            continue;
        };
        let volume = match traffic.as_str() {
            "0 to 999" => TrafficVolume::UpTo1000,
            "1000 to 1999" => TrafficVolume::UpTo2000,
            "2000 to 3999" => TrafficVolume::UpTo4000,
            "4000+" => TrafficVolume::Over4000,
            _ => bail!("Unknown Traffic.volume.category {traffic}"),
        };

        let mut geom: LineString = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);

        source_geometry.push(geom);
        source_data.push(volume);
    }
    if source_geometry.is_empty() {
        bail!("No traffic volumes detected; input is broken");
    }

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
    for idx in 0..graph.roads.len() {
        // There are issues with small dead-end service roads next to big roads and car-free roads
        // having high traffic assigned. Sometimes the problem is map matching, sometimes the
        // source dataset is wrong. Override in some cases.
        if matches!(
            highways[idx],
            Highway::Service
                | Highway::Footway
                | Highway::Cycleway
                | Highway::Pedestrian
                | Highway::Path
        ) {
            results.push(TrafficVolume::UpTo1000);
            continue;
        }
        results
            .push(get_anime_match(&matches, &source_data, idx).unwrap_or(TrafficVolume::UpTo1000));
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
    let combined_2way_idx = layer.defn().field_index("combined_2way")?;
    let cross_section_profile_idx = layer.defn().field_index("cross_section_profile")?;
    let edge_to_edge_width_idx = layer.defn().field_index("edge_to_edge_width")?;

    let mut source_geometry = Vec::new();
    let mut source_data = Vec::new();
    for input in layer.features() {
        let mut geom: LineString = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);
        let Some(combined_2way) = input.field_as_string(combined_2way_idx)? else {
            // Some are just missing
            continue;
        };
        let segregated_fits =
            combined_2way == "Absolute minimum" || combined_2way == "Desirable minimum";
        let cross_section_profile = input
            .field_as_string(cross_section_profile_idx)?
            .unwrap_or_else(|| "no data".to_string());
        let edge_to_edge_width =
            input.field_as_integer(edge_to_edge_width_idx)?.unwrap_or(0) as usize;

        source_geometry.push(geom);
        source_data.push(Streetspace {
            segregated_fits,
            cross_section_profile,
            edge_to_edge_width,
        });
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

// The output is per road. If the road is part of the coherent network, what tier is it?
fn read_coherent_network(
    path: &str,
    graph: &Graph,
    highways: &Vec<Highway>,
    timer: &mut Timer,
) -> Result<Vec<Option<Tier>>> {
    // Read all relevant lines
    timer.step("read coherent network");
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);
    let road_function_npt_idx = layer.defn().field_index("road_function_npt")?;

    let mut source_geometry = Vec::new();
    let mut source_data = Vec::new();
    for input in layer.features() {
        let Some(function) = input.field_as_string(road_function_npt_idx)? else {
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
            _ => bail!("read_coherent_network found something besides a LS or MLS"),
        }
    }

    timer.step("match coherent network");
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
        // The coherent network can never be on the motorway, but in Glasgow, it's hard to
        // distinguish some of the parallel roads
        if matches!(highways[idx], Highway::Motorway) {
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
    let id_idx = layer.defn().field_index("id")?;
    let name_idx = layer.defn().field_index("name")?;

    let mut greenspaces = Vec::new();
    for input in layer.features() {
        let Some(id) = input.field_as_string(id_idx)? else {
            bail!("Missing id");
        };
        let name = input.field_as_string(name_idx)?;
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

fn find_streets_by_greenspace(
    path: &str,
    boundary_wgs84: &MultiPolygon,
    graph: &Graph,
    timer: &mut Timer,
) -> Result<Vec<bool>> {
    timer.step("find streets near greenspaces");
    let profile = graph.profile_names["bicycle_direct"];
    let mut is_attractive: Vec<bool> = std::iter::repeat(false).take(graph.roads.len()).collect();

    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);

    for input in layer.features() {
        let Ok(mut geom): std::result::Result<MultiPolygon, geo_types::Error> =
            input.geometry().unwrap().to_geo()?.try_into()
        else {
            continue;
        };
        if !boundary_wgs84.intersects(&geom) {
            continue;
        }
        graph.mercator.to_mercator_in_place(&mut geom);

        // Buffer the greenspace a bit, then see if that intersects any roads, even at one point
        let Ok(buffered) = buffer_polygon(&geom, 10.0) else {
            continue;
        };
        // TODO rstar can't directly calculate a MultiPolygon envelope
        let bbox: Rect = buffered.bounding_rect().unwrap().into();
        let envelope = AABB::from_corners(
            Point::new(bbox.min().x, bbox.min().y),
            Point::new(bbox.max().x, bbox.max().y),
        );

        for obj in graph.routers[profile.0]
            .closest_road
            .locate_in_envelope_intersecting(&envelope)
        {
            let road = &graph.roads[obj.data.0];
            if !is_attractive[road.id.0] && road.linestring.intersects(&buffered) {
                is_attractive[road.id.0] = true;
            }
        }
    }

    Ok(is_attractive)
}

fn read_access_points(path: &str, graph: &Graph) -> Result<HashMap<String, Vec<Point>>> {
    let dataset = Dataset::open(path)?;
    let mut layer = dataset.layer(0)?;
    let b = &graph.mercator.wgs84_bounds;
    layer.set_spatial_filter_rect(b.min().x, b.min().y, b.max().x, b.max().y);
    let site_id_idx = layer.defn().field_index("site_id")?;

    let mut pts_per_site = HashMap::new();
    for input in layer.features() {
        let Some(id) = input.field_as_string(site_id_idx)? else {
            bail!("Missing site_id");
        };
        let mut geom: Point = input.geometry().unwrap().to_geo()?.try_into()?;
        graph.mercator.to_mercator_in_place(&mut geom);

        pts_per_site.entry(id).or_insert_with(Vec::new).push(geom);
    }
    Ok(pts_per_site)
}

fn get_anime_match<T: Clone>(
    matches: &anime::MatchesMap,
    source_data: &Vec<T>,
    target_idx: usize,
) -> Option<T> {
    let candidates = matches.get(&target_idx)?;
    // Use the one with the most shared length
    let c = candidates
        .into_iter()
        .max_by_key(|c| (c.shared_len * 1000.0) as usize)?;
    Some(source_data[c.source_index].clone())
}

// TODO By michaelkirk, copied from ltn repo -- will wind up upstreamed in geo likely soon
/// Buffers a polygon, returning the largest of the output Polygons
///
/// Buffering can leave floating artifacts.
/// I haven't investigated why yet, but dealing with it is simple enough.
/// i_overlay offers a relevant sounding `min_area` parameter, but it's not currently exposed
/// by geo's buffer integration.
fn buffer_polygon(area: &impl Buffer<Scalar = f64>, distance: f64) -> Result<Polygon> {
    let merged = area.buffer(distance);

    let area_polygons = merged.0.into_iter().map(|p| (p.unsigned_area(), p));

    // Buffering can leave floating artifacts.
    //
    // I haven't investigated why yet, but dealing with it is simple enough.
    // i_overaly offers a relevant sounding `min_area` parameter, but it's not currently exposed
    // by geo's buffer integration.
    //
    // For perspective, a 60m (interior) roundabout has an area around 2800m²
    // We may have to tweak or parameterize this if we encounter errors with the current value.
    let buffering_artifact_threshold_m2 = 1000.;
    let mut merged_boundaries: Vec<_> = area_polygons
        .filter(|(area, _polygon)| *area > buffering_artifact_threshold_m2)
        .collect();

    let polygon = match merged_boundaries.len() {
        0 => bail!("Empty boundary"),
        1 => merged_boundaries.pop().expect("verified non-empty").1,
        _ => {
            bail!("All included boundaries must be adjacent");
        }
    };
    Ok(polygon)
}

// TODO Unit test
fn get_speed_mph(hwy: Highway, tags: &Tags) -> usize {
    if tags.is("maxspeed", "national") {
        return if matches!(hwy, Highway::Motorway) {
            70
        } else {
            60
        };
    }

    if let Some(maxspeed) = tags.get("maxspeed") {
        if let Some(mph) = maxspeed
            .strip_suffix(" mph")
            .and_then(|x| x.parse::<usize>().ok())
        {
            return mph;
        }
    }

    // TODO Check these against osmactive
    match hwy {
        Highway::Motorway => 70,
        Highway::Trunk => 60,
        Highway::Primary => 40,
        Highway::Secondary | Highway::Tertiary | Highway::Residential => 30,
        Highway::Service | Highway::Unclassified => 10,
        Highway::LivingStreet => 15,
        // TODO What should these do?
        Highway::Footway | Highway::Cycleway | Highway::Pedestrian | Highway::Path => 10,
    }
}
