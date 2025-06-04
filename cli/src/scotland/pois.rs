use std::collections::HashSet;

use anyhow::Result;
use geo::{Area, BooleanOps, Centroid, Contains, Intersects, MultiPolygon, Point, Polygon};
use graph::{Graph, RoadID};
use serde::Deserialize;

use crate::common;
use backend::places::{GPHospital, RailwayStation, School, Settlement, TownCentre};

pub fn load_schools(gj: &str, boundary_wgs84: &MultiPolygon, graph: &Graph) -> Result<Vec<School>> {
    let mut schools = Vec::new();
    for obj in geojson::de::deserialize_feature_collection_str_to_vec::<SchoolGJ>(gj)? {
        if boundary_wgs84.contains(&obj.geometry) {
            schools.push(common::make_school(
                &graph,
                obj.geometry,
                obj.name,
                obj.r#type,
                obj.pupils as usize,
            ));
        }
    }
    info!("Matched {} schools", schools.len());
    Ok(schools)
}

#[derive(Deserialize)]
struct SchoolGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: Point,
    r#type: String,
    name: String,
    pupils: f64,
}

pub fn load_gps_hospitals(
    gp_gj: &str,
    hospitals_gj: &str,
    boundary_wgs84: &MultiPolygon,
    graph: &Graph,
) -> Result<Vec<GPHospital>> {
    let mut gp_hospitals = Vec::new();
    for (gj, kind) in [(gp_gj, "GP"), (hospitals_gj, "hospital")] {
        for obj in geojson::de::deserialize_feature_collection_str_to_vec::<GPHospitalGJ>(gj)? {
            if boundary_wgs84.contains(&obj.geometry) {
                gp_hospitals.push(common::make_gp_hospital(
                    &graph,
                    obj.geometry,
                    obj.name,
                    kind.to_string(),
                ));
            }
        }
    }
    info!("Matched {} GPs/hospitals", gp_hospitals.len());
    Ok(gp_hospitals)
}

#[derive(Deserialize)]
struct GPHospitalGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: Point,
    name: String,
}

pub fn load_railway_stations(
    gj: &str,
    boundary_wgs84: &MultiPolygon,
    graph: &Graph,
) -> Result<Vec<RailwayStation>> {
    let mut railway_stations = Vec::new();
    for obj in geojson::de::deserialize_feature_collection_str_to_vec::<RailwayStationGJ>(gj)? {
        if boundary_wgs84.contains(&obj.geometry) {
            railway_stations.push(common::make_railway_station(&graph, obj.geometry, obj.name));
        }
    }
    info!("Matched {} railway stations", railway_stations.len());
    Ok(railway_stations)
}

#[derive(Deserialize)]
struct RailwayStationGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: Point,
    name: Option<String>,
}

pub fn load_town_centres(
    gj: &str,
    boundary_wgs84: &MultiPolygon,
    graph: &Graph,
) -> Result<Vec<TownCentre>> {
    let boundary_mercator = graph.mercator.to_mercator(boundary_wgs84);

    let mut town_centres = Vec::new();
    for x in geojson::de::deserialize_feature_collection_str_to_vec::<TownCentreGJ>(gj)? {
        if boundary_wgs84.intersects(&x.geometry) {
            let polygon = graph.mercator.to_mercator(&x.geometry);

            // How much of the zone intersects the study area?
            let overlap = boundary_mercator.intersection(&polygon);
            let ratio_in_boundary = overlap.unsigned_area() / polygon.unsigned_area();
            if ratio_in_boundary < 0.1 {
                info!(
                    "Skipping town centre {:?} because only {}% of it overlaps the boundary",
                    x.name,
                    ratio_in_boundary * 100.0
                );
                continue;
            }

            // All intersecting roads
            // TODO Could rtree to speed up
            let mut roads: HashSet<RoadID> = HashSet::new();
            for (idx, road) in graph.roads.iter().enumerate() {
                if polygon.intersects(&road.linestring) {
                    roads.insert(RoadID(idx));
                }
            }
            if roads.is_empty() {
                info!("Town centre {:?} doesn't intersect any road. Just snapping to one arbitrary close road.", x.name);
                let centroid = polygon.centroid().unwrap();
                roads.insert(
                    graph
                        .snap_to_road(centroid.into(), graph.profile_names["bicycle_direct"])
                        .road,
                );
            }

            town_centres.push(TownCentre {
                polygon,
                name: x.name,
                roads,
            });
        }
    }
    info!("Matched {} town centres", town_centres.len());
    Ok(town_centres)
}

#[derive(Deserialize)]
struct TownCentreGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: Polygon,
    name: Option<String>,
}

pub fn load_settlements(
    gj: &str,
    boundary_wgs84: &MultiPolygon,
    graph: &Graph,
) -> Result<Vec<Settlement>> {
    let boundary_mercator = graph.mercator.to_mercator(boundary_wgs84);

    let mut settlements = Vec::new();
    for x in geojson::de::deserialize_feature_collection_str_to_vec::<SettlementGJ>(gj)? {
        if boundary_wgs84.intersects(&x.geometry) {
            let settlement_mercator = graph.mercator.to_mercator(&x.geometry);
            // Clip the settlement to the study area
            let mut settlement_pieces = boundary_mercator.intersection(&settlement_mercator);
            settlement_pieces.0.retain(|polygon| {
                // Settlement polygons are more precise than the simplified local authority
                // boundaries. Rather than switch to the precise LA boundaries (from
                // https://data.spatialhub.scot/dataset/local_authority_boundaries-is/resource/d24c5735-0f1c-4819-a6bd-dbfeb93bd8e4)
                // and incur a file size hit, just check for a minimum area of the clipped settlements. By manual inspection, this threshold is reasonable.
                let keep = polygon.unsigned_area() >= 10_000.0;
                if !keep {
                    info!(
                        "Skipping settlement {:?} because it's tiny after being clipped",
                        x.name
                    );
                }
                keep
            });

            let n = settlement_pieces.0.len();
            for (idx, polygon) in settlement_pieces.0.into_iter().enumerate() {
                // All intersecting roads
                // TODO Could rtree to speed up
                let mut roads: HashSet<RoadID> = HashSet::new();
                for (idx, road) in graph.roads.iter().enumerate() {
                    if polygon.intersects(&road.linestring) {
                        roads.insert(RoadID(idx));
                    }
                }
                if roads.is_empty() {
                    // Just log it; there are some settlement pieces on empty islands
                    error!("Settlement {:?} doesn't snap to any roads", x.name);
                }

                settlements.push(Settlement {
                    polygon,
                    name: if let Some(ref name) = x.name {
                        if n > 1 {
                            Some(format!("{name} ({} / {n})", idx + 1))
                        } else {
                            Some(name.clone())
                        }
                    } else {
                        None
                    },
                    population: x.population as usize,
                    roads,
                });
            }
        }
    }
    info!("Matched {} settlements", settlements.len());
    Ok(settlements)
}

#[derive(Deserialize)]
struct SettlementGJ {
    #[serde(deserialize_with = "geojson::de::deserialize_geometry")]
    geometry: MultiPolygon,
    name: Option<String>,
    #[serde(rename = "Population")]
    population: f64,
}
