use std::collections::{HashMap, HashSet};

use anyhow::Result;
use geo::{
    Area, BooleanOps, BoundingRect, Centroid, Coord, Intersects, LineString, MultiPolygon, Point,
    Polygon, Rect,
};
use graph::{Graph, RoadID, Timer};
use rstar::AABB;
use serde::Deserialize;
use utils::Tags;

use crate::{common, disconnected::remove_disconnected_components};
use backend::{places::DataZone, Highway, MapModel, TrafficVolume};

pub fn create(
    study_area_name: String,
    input_bytes: &[u8],
    boundary_gj: &str,
    timer: &mut Timer,
) -> Result<MapModel> {
    let mut pois = OsmPOIs::default();

    info!("Creating MapModel for {study_area_name}");
    let graph = Graph::new(
        input_bytes,
        &mut pois,
        Box::new(remove_disconnected_components),
        Box::new(|_, _| Ok(())),
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

    let schools = pois
        .schools
        .into_iter()
        .map(|(pt, name)| common::make_school(&graph, pt, name, String::new(), 0))
        .collect();

    let gp_hospitals =
        pois.gps
            .into_iter()
            .map(|(pt, name)| common::make_gp_hospital(&graph, pt, name, "GP".to_string()))
            .chain(pois.hospitals.into_iter().map(|(pt, name)| {
                common::make_gp_hospital(&graph, pt, name, "hospital".to_string())
            }))
            .collect();

    let railway_stations = pois
        .railway_stations
        .into_iter()
        .map(|(pt, name)| common::make_railway_station(&graph, pt, Some(name)))
        .collect();

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

    let gradients = common::read_gradients(
        "../data_prep/england/inputs/UK-dem-50m-4326.tif",
        &graph,
        timer,
    )?;

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
    let mut densities = Vec::new();
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
                population: x.population as usize,
                area_km2,
                roads,
                density_quintile: 0,
                centroid_wgs84,

                x1: (bbox.min().x * 100.0) as i64,
                y1: (bbox.min().y * 100.0) as i64,
                x2: (bbox.max().x * 100.0) as i64,
                y2: (bbox.max().y * 100.0) as i64,
            });

            densities.push((x.population / area_km2) as usize);
        }
    }

    let stats = common::Quintiles::new(&densities);
    for zone in &mut zones {
        zone.density_quintile = stats.quintile(((zone.population as f64) / zone.area_km2) as usize);
    }

    info!("Matched {} population zones", zones.len());
    Ok(zones)
}

#[derive(Deserialize)]
struct OutputAreaGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: MultiPolygon,
    name: String,
    population: f64,
}

#[derive(Default)]
struct OsmPOIs {
    schools: Vec<(Point, String)>,
    gps: Vec<(Point, String)>,
    hospitals: Vec<(Point, String)>,
    railway_stations: Vec<(Point, String)>,
}

// TODO Needs more thorough work to handle all cases, handle relations (like schools over many
// buildings)
impl utils::osm2graph::OsmReader for OsmPOIs {
    fn node(&mut self, _: osm_reader::NodeID, pt: Coord, tags: Tags) {
        if let Some(name) = is_school(&tags) {
            self.schools.push((pt.into(), name));
        } else if let Some(name) = is_gp(&tags) {
            self.gps.push((pt.into(), name));
        } else if let Some(name) = is_hospital(&tags) {
            self.hospitals.push((pt.into(), name));
        } else if let Some(name) = is_railway_station(&tags) {
            self.railway_stations.push((pt.into(), name));
        }
    }

    fn way(
        &mut self,
        _: osm_reader::WayID,
        node_ids: &Vec<osm_reader::NodeID>,
        node_mapping: &HashMap<osm_reader::NodeID, Coord>,
        tags: &Tags,
    ) {
        // It's not critical to handle holes just to calculate a centroid
        let get_pt = || {
            let exterior = LineString::new(node_ids.into_iter().map(|n| node_mapping[n]).collect());
            Polygon::new(exterior, Vec::new()).centroid().unwrap()
        };

        if let Some(name) = is_school(&tags) {
            self.schools.push((get_pt(), name));
        } else if let Some(name) = is_gp(&tags) {
            self.gps.push((get_pt(), name));
        } else if let Some(name) = is_hospital(&tags) {
            self.hospitals.push((get_pt(), name));
        }
    }

    // TODO Handle these
    fn relation(
        &mut self,
        _: osm_reader::RelationID,
        _members: &Vec<(String, osm_reader::OsmID)>,
        _tags: &Tags,
    ) {
    }
}

fn is_school(tags: &Tags) -> Option<String> {
    if tags.is("amenity", "school") {
        // TODO Some have capacity (pupils)
        // TODO Could determine kind from the name sometimes
        return Some(
            tags.get("name")
                .cloned()
                .unwrap_or_else(|| "Unnamed school".to_string()),
        );
    }
    None
}

fn is_gp(tags: &Tags) -> Option<String> {
    if tags.is("amenity", "doctors") {
        return Some(
            tags.get("name")
                .cloned()
                .unwrap_or_else(|| "Unnamed GP".to_string()),
        );
    }
    None
}

fn is_hospital(tags: &Tags) -> Option<String> {
    if tags.is("amenity", "hospital") {
        return Some(
            tags.get("name")
                .cloned()
                .unwrap_or_else(|| "Unnamed hospital".to_string()),
        );
    }
    None
}

fn is_railway_station(tags: &Tags) -> Option<String> {
    if tags.is("railway", "station") {
        return Some(
            tags.get("name")
                .cloned()
                .unwrap_or_else(|| "Unnamed railway station".to_string()),
        );
    }
    None
}
