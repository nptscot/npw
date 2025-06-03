use std::io::BufWriter;

use anime::Anime;
use anyhow::{bail, Result};
use clap::Parser;
use fs_err::File;
use geo::MultiPolygon;
use graph::{Graph, Timer};
use utils::Tags;

use backend::{Highway, TrafficVolume};

mod disconnected;
mod england;
mod scotland;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    country: String,

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
    let osm_bytes = fs_err::read(&args.input)?;
    let boundary_gj = fs_err::read_to_string(&args.boundary)?;
    let study_area_name = args
        .output
        .split("/")
        .last()
        .unwrap()
        .strip_suffix(".bin")
        .unwrap()
        .to_string();
    let model = match args.country.as_ref() {
        "scotland" => scotland::create(study_area_name, &osm_bytes, &boundary_gj, &mut timer)?,
        "england" => england::create(study_area_name, &osm_bytes, &boundary_gj, &mut timer)?,
        x => bail!("Unknown country {x}"),
    };

    timer.step("writing");
    let writer = BufWriter::new(File::create(&args.output)?);
    bincode::serialize_into(writer, &model)?;

    fs_err::write(
        &args.stats_output,
        serde_json::to_string(&model.get_baseline_stats())?,
    )?;

    timer.done();
    Ok(())
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

// TODO Unit test
fn get_speed_mph(hwy: Highway, tags: &Tags) -> usize {
    if tags.is("maxspeed", "national") {
        // No motorways are included
        return 60;
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
        Highway::Trunk => 60,
        Highway::Primary => 40,
        Highway::Secondary | Highway::Tertiary | Highway::Residential => 30,
        Highway::Service | Highway::Unclassified => 10,
        Highway::LivingStreet => 15,
        // TODO What should these do?
        Highway::Footway | Highway::Cycleway | Highway::Pedestrian | Highway::Path => 10,
    }
}

fn handle_parallel_roads(
    highways: &Vec<Highway>,
    traffic_volumes: &mut Vec<TrafficVolume>,
    speeds: &mut Vec<usize>,
    graph: &Graph,
) {
    // Sources are roads with motor vehicles
    let mut source_geometry = Vec::new();
    let mut source_data = Vec::new();

    let mut targets = Vec::new();
    for (idx, road) in graph.roads.iter().enumerate() {
        if highways[idx].has_motor_vehicles() {
            source_geometry.push(road.linestring.clone());
            source_data.push(idx);
        } else {
            targets.push(idx);
        }
    }

    let distance_tolerance = 15.0;
    let angle_tolerance = 10.0;
    let matches = Anime::new(
        source_geometry.into_iter(),
        targets
            .iter()
            .map(|idx| graph.roads[*idx].linestring.clone()),
        distance_tolerance,
        angle_tolerance,
    )
    .matches
    .take()
    .unwrap();

    let debug = false;
    let mut features = Vec::new();

    for (anime_target_idx, target_idx) in targets.into_iter().enumerate() {
        if let Some((source_idx, shared_len)) =
            get_anime_match(&matches, &source_data, anime_target_idx)
        {
            // TODO This isn't correct; it's over 100% sometimes. Unclear what this represents. But
            // it still seems to be 0 when the target road is just incident to the source and not
            // actually parallel.
            let pct_shared_len = 100.0 * (shared_len / graph.roads[target_idx].length_meters);
            if pct_shared_len < 1.0 {
                continue;
            }

            if debug {
                let mut f = graph
                    .mercator
                    .to_wgs84_gj(&graph.roads[target_idx].linestring);
                f.set_property("way", graph.roads[source_idx].way.to_string());
                f.set_property(
                    "pct_shared_len",
                    100.0 * (shared_len / graph.roads[target_idx].length_meters),
                );
                features.push(f);
            }

            // Copy the speed/volume from the big road
            speeds[target_idx] = speeds[source_idx];
            traffic_volumes[target_idx] = traffic_volumes[source_idx];
        }
    }

    if debug {
        fs_err::write(
            "debug_parallel.geojson",
            serde_json::to_string(&geojson::GeoJson::from(features)).unwrap(),
        )
        .unwrap();
    }
}

// Returns the data and shared length
fn get_anime_match<T: Clone>(
    matches: &anime::MatchesMap,
    source_data: &Vec<T>,
    target_idx: usize,
) -> Option<(T, f64)> {
    let candidates = matches.get(&target_idx)?;
    // Use the one with the most shared length
    let c = candidates
        .into_iter()
        .max_by_key(|c| (c.shared_len * 1000.0) as usize)?;
    Some((source_data[c.source_index].clone(), c.shared_len))
}
