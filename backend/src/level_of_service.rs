use anyhow::Result;
use enum_map::Enum;
use geojson::FeatureCollection;
use graph::RoadID;
use serde::Serialize;
use utils::Tags;

use crate::{Highway, InfraType, MapModel};

#[derive(Clone, Copy, Debug, PartialEq, Enum, Serialize)]
pub enum LevelOfService {
    High,
    Medium,
    Low,
    ShouldNotBeUsed,
}

impl MapModel {
    pub fn render_level_of_service(&self) -> Result<String> {
        let mut features = Vec::new();
        for (idx, road) in self.graph.roads.iter().enumerate() {
            let id = RoadID(idx);

            let mut f = self.graph.mercator.to_wgs84_gj(&road.linestring);
            f.set_property("los", serde_json::to_value(self.los[idx])?);
            f.set_property("infra_type", serde_json::to_value(self.get_infra_type(id))?);
            f.set_property("traffic", self.traffic_volumes[idx]);
            f.set_property("speed", self.speeds[idx]);
            // TODO Abusing this here; need to consolidate the output layers
            f.set_property("gradient", self.gradients[idx]);
            features.push(f);
        }

        Ok(serde_json::to_string(&FeatureCollection {
            features,
            bbox: None,
            foreign_members: None,
        })?)
    }

    pub fn calculate_level_of_service(&self, r: RoadID) -> LevelOfService {
        get_level_of_service(
            self.get_infra_type(r),
            self.speeds[r.0],
            self.traffic_volumes[r.0],
        )
    }

    /// Picks the "cheapest" InfraType that makes a road be classified as high LoS
    pub fn best_infra_type(&self, r: RoadID) -> InfraType {
        let speed = self.speeds[r.0];
        let traffic = self.traffic_volumes[r.0];

        // TODO Is this order correct, and do we want to use all of these?
        for infra_type in [
            InfraType::MixedTraffic,
            InfraType::CycleLane,
            InfraType::SegregatedNarrow,
        ] {
            if get_level_of_service(infra_type, speed, traffic) == LevelOfService::High {
                return infra_type;
            }
        }

        // Speed+volume mean there's only one solution
        InfraType::SegregatedWide
    }
}

/// This follows table 3.2 from https://www.transport.gov.scot/media/50323/cycling-by-design-update-2019-final-document-15-september-2021-1.pdf as closely as possible
pub fn get_level_of_service(infra_type: InfraType, speed: usize, traffic: usize) -> LevelOfService {
    match infra_type {
        InfraType::MixedTraffic => {
            if speed <= 20 {
                if traffic < 2000 {
                    LevelOfService::High
                } else if traffic < 4000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else if speed <= 30 {
                if traffic < 1000 {
                    LevelOfService::High
                } else if traffic < 2000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else if speed <= 40 {
                if traffic < 1000 {
                    LevelOfService::Medium
                } else if traffic < 2000 {
                    LevelOfService::Low
                } else {
                    LevelOfService::ShouldNotBeUsed
                }
            } else if speed <= 60 {
                // Both 50 and 60mph cases
                if traffic < 1000 {
                    LevelOfService::Low
                } else {
                    LevelOfService::ShouldNotBeUsed
                }
            } else {
                LevelOfService::ShouldNotBeUsed
            }
        }

        // TODO Not sure how this maps to the CbD category. Since this is the best option on
        // big roads, treat it as "Detached or Remote Cycle Track"
        InfraType::SegregatedWide => LevelOfService::High,

        // "Detached or Remote Cycle Track"
        InfraType::OffRoad => LevelOfService::High,

        // TODO confirm both cases: "Stepped or Footway Level Cycle Track"?
        InfraType::SegregatedNarrow | InfraType::SharedFootway => {
            if speed <= 20 {
                LevelOfService::High
            } else if speed <= 30 {
                if traffic < 4000 {
                    LevelOfService::High
                } else {
                    LevelOfService::Medium
                }
            } else if speed <= 40 {
                LevelOfService::Medium
            } else if speed <= 50 {
                if traffic < 1000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else {
                LevelOfService::Low
            }
        }

        InfraType::CycleLane => {
            if speed <= 20 {
                if traffic < 4000 {
                    LevelOfService::High
                } else {
                    LevelOfService::Medium
                }
            } else if speed <= 30 {
                if traffic < 1000 {
                    LevelOfService::High
                } else if traffic < 4000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else if speed <= 40 {
                if traffic < 1000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else if speed <= 50 {
                LevelOfService::Low
            } else if speed <= 60 {
                if traffic < 1000 {
                    LevelOfService::Low
                } else {
                    LevelOfService::ShouldNotBeUsed
                }
            } else {
                LevelOfService::ShouldNotBeUsed
            }
        }
    }
}

// TODO Unit test
pub fn get_speed_mph(tags: &Tags) -> usize {
    let hwy = Highway::classify(tags).unwrap();

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
        Highway::Secondary | Highway::Tertiary => 30,
        Highway::Residential | Highway::Service | Highway::Unclassified => 20,
        Highway::LivingStreet => 15,
        // TODO What should these do?
        Highway::Footway | Highway::Cycleway | Highway::Pedestrian | Highway::Path => 10,
    }
}
