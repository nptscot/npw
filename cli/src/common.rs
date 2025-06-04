use std::collections::HashMap;

use anime::Anime;
use anyhow::{bail, Result};
use fs_err::File;
use geo::MultiPolygon;
use graph::Graph;
use serde::Deserialize;
use utils::Tags;

use backend::{Highway, TrafficVolume};

pub fn read_multipolygon(gj_string: &str) -> Result<MultiPolygon> {
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
pub fn get_speed_mph(hwy: Highway, tags: &Tags) -> usize {
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

pub fn handle_parallel_roads(
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
pub fn get_anime_match<T: Clone>(
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

pub fn read_commute_desire_lines_csv(
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

#[derive(Debug)]
pub struct Quintiles {
    // 20% of values are >= this amount
    pub quintile1: usize,
    // 40% of values are >= this amount
    pub quintile2: usize,
    quintile3: usize,
    quintile4: usize,
    // quintile5 is the minimum value
    pub total_quintile_sums: [usize; 5],
    // Just show in Debug
    #[allow(dead_code)]
    max: usize,
}

impl Quintiles {
    /// 0s are ignored
    pub fn new(values: &Vec<usize>) -> Self {
        let mut sorted = values.clone();
        sorted.retain(|x| *x > 0);
        sorted.sort();
        sorted.reverse();
        // Rounding up may be a little wrong, but it's OK, because we don't calculate quintile5
        // explicitly
        let n = ((sorted.len() as f64) / 5.0).ceil() as usize;

        let mut quintiles = Self {
            quintile1: sorted[n],
            quintile2: sorted[n * 2],
            quintile3: sorted[n * 3],
            quintile4: sorted[n * 4],
            total_quintile_sums: [0; 5],
            max: sorted[0],
        };

        for value in sorted {
            let quintile = quintiles.quintile(value);
            quintiles.total_quintile_sums[quintile - 1] += value;
        }

        quintiles
    }

    // Returns [1, 5]
    pub fn quintile(&self, value: usize) -> usize {
        if value >= self.quintile1 {
            1
        } else if value >= self.quintile2 {
            2
        } else if value >= self.quintile3 {
            3
        } else if value >= self.quintile4 {
            4
        } else {
            5
        }
    }
}
