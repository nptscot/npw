use std::collections::HashMap;

use anyhow::Result;
use graph::{Graph, Timer};
use log::info;

use crate::disconnected::remove_disconnected_components;
use backend::{Highway, MapModel, TrafficVolume};

pub fn create(
    study_area_name: String,
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
    let boundary_wgs84 = crate::read_multipolygon(boundary_gj)?;

    let data_zones: Vec<backend::places::DataZone> = Vec::new();
    let _zone_ids: HashMap<String, usize> = data_zones
        .iter()
        .enumerate()
        .map(|(idx, zone)| (zone.id.clone(), idx))
        .collect();

    let commute_desire_lines = Vec::new();
    let other_desire_lines = Vec::new();

    let schools = Vec::new();

    let gp_hospitals = Vec::new();

    let railway_stations = Vec::new();

    let greenspaces = Vec::new();

    let town_centres = Vec::new();

    let settlements = vec![backend::places::Settlement {
        polygon: graph.boundary_polygon.clone(),
        name: None,
        // TODO Make it optional
        population: 0,
        roads: graph.roads.iter().map(|r| r.id).collect(),
    }];

    let highways: Vec<_> = graph
        .roads
        .iter()
        .map(|r| Highway::classify(&r.osm_tags).unwrap())
        .collect();

    // TODO Temporary
    let mut traffic_volumes = highways
        .iter()
        .map(|hwy| match hwy {
            Highway::Trunk | Highway::Primary | Highway::Secondary => TrafficVolume::Over4000,
            Highway::Tertiary => TrafficVolume::UpTo4000,
            Highway::Residential => TrafficVolume::UpTo2000,
            _ => TrafficVolume::UpTo1000,
        })
        .collect();

    let mut speeds = graph
        .roads
        .iter()
        .enumerate()
        .map(|(idx, r)| crate::get_speed_mph(highways[idx], &r.osm_tags))
        .collect();

    crate::handle_parallel_roads(&highways, &mut traffic_volumes, &mut speeds, &graph);

    let street_space = std::iter::repeat(None).take(highways.len()).collect();

    let is_attractive = std::iter::repeat(false).take(highways.len()).collect();

    let gradients = std::iter::repeat(0.0).take(highways.len()).collect();

    Ok(MapModel::create(
        study_area_name,
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
        street_space,
        is_attractive,
        gradients,
        timer,
    )?)
}
