use std::collections::{HashMap, HashSet};

use anyhow::Result;
use geo::{Area, BooleanOps, BoundingRect, Centroid, Intersects, MultiPolygon, Point, Rect};
use graph::{Graph, RoadID, Timer};
use rstar::AABB;
use serde::Deserialize;

use crate::{common, disconnected::remove_disconnected_components};
use backend::{places::DataZone, Highway, MapModel, TrafficVolume};

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
    let boundary_wgs84 = common::read_multipolygon(boundary_gj)?;

    // TODO Rename these to be a bit more general
    timer.step("loading data zones");
    let data_zones = load_data_zones(
        &fs_err::read_to_string("../data_prep/england/inputs/zones.geojson")?,
        &boundary_wgs84,
        &graph,
    )?;
    let zone_ids: HashMap<String, usize> = data_zones
        .iter()
        .enumerate()
        .map(|(idx, zone)| (zone.id.clone(), idx))
        .collect();

    timer.step("loading commute desire lines");
    let commute_desire_lines = common::read_commute_desire_lines_csv(
        "../data_prep/england/inputs/od_commute.csv",
        &zone_ids,
    )?;
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
        .map(|(idx, r)| common::get_speed_mph(highways[idx], &r.osm_tags))
        .collect();

    common::handle_parallel_roads(&highways, &mut traffic_volumes, &mut speeds, &graph);

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

// TODO The input doesn't have IMD or population yet
fn load_data_zones(
    gj: &str,
    boundary_wgs84: &MultiPolygon,
    graph: &Graph,
) -> Result<Vec<DataZone>> {
    let profile = graph.profile_names["bicycle_direct"];
    let boundary_mercator = graph.mercator.to_mercator(boundary_wgs84);

    let mut zones = Vec::new();
    for x in geojson::de::deserialize_feature_collection_str_to_vec::<OutputAreaGJ>(gj)? {
        if boundary_wgs84.intersects(&x.geometry) {
            let polygon = graph.mercator.to_mercator(&x.geometry);

            // How much of the zone intersects the study area?
            let overlap = boundary_mercator.intersection(&polygon);
            let ratio_in_boundary = overlap.unsigned_area() / polygon.unsigned_area();
            if ratio_in_boundary < 0.1 {
                info!(
                    "Skipping population zone {} because only {}% of it overlaps the boundary",
                    x.name,
                    ratio_in_boundary * 100.0
                );
                continue;
            }

            // TODO rstar can't directly calculate a MultiPolygon envelope
            let bbox: Rect = polygon.bounding_rect().unwrap().into();
            let envelope = AABB::from_corners(
                Point::new(bbox.min().x, bbox.min().y),
                Point::new(bbox.max().x, bbox.max().y),
            );

            // All intersecting roads
            let roads: HashSet<RoadID> = graph.routers[profile.0]
                .closest_road
                .locate_in_envelope_intersecting(&envelope)
                .map(|obj| obj.data)
                .collect();

            // Of the entire data zone, not just the part clipped to the study area
            let area_km2 = polygon.unsigned_area() / 10.0e6;

            let centroid = overlap.centroid().unwrap();
            let centroid_wgs84 = graph.mercator.pt_to_wgs84(centroid.into());

            zones.push(DataZone {
                polygon,
                id: x.name,
                imd_rank: 0,
                imd_percentile: 0,
                population: 0,
                area_km2,
                roads,
                density_quintile: 0,
                centroid_wgs84,

                x1: (bbox.min().x * 100.0) as i64,
                y1: (bbox.min().y * 100.0) as i64,
                x2: (bbox.max().x * 100.0) as i64,
                y2: (bbox.max().y * 100.0) as i64,
            });
        }
    }

    info!("Matched {} population zones", zones.len());
    Ok(zones)
}

#[derive(Deserialize)]
struct OutputAreaGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: MultiPolygon,
    name: String,
}
