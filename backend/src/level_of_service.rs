use enum_map::Enum;
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
    pub fn calculate_level_of_service(&self, r: RoadID) -> LevelOfService {
        if self.override_infra_type[r.0] {
            return LevelOfService::High;
        }

        get_level_of_service(
            self.get_infra_type(r),
            self.speeds[r.0],
            self.traffic_volumes[r.0],
            self.within_settlement[r.0],
        )
    }

    /// Picks the "cheapest" InfraType that makes a road be classified as high LoS (or as high as
    /// possible)
    pub fn best_infra_type(&self, r: RoadID) -> InfraType {
        // Easy first case -- this is always high LoS
        if self.is_offroad[r.0] {
            return InfraType::OffRoad;
        }

        let speed = self.speeds[r.0];
        let traffic = self.traffic_volumes[r.0];
        let within_settlement = self.within_settlement[r.0];

        for infra_type in [
            InfraType::MixedTraffic,
            // TODO Revisit CbD definitions. For now, never suggest.
            //InfraType::CycleLane,
            InfraType::SharedFootway,
        ] {
            if get_level_of_service(infra_type, speed, traffic, within_settlement)
                == LevelOfService::High
            {
                return infra_type;
            }
        }

        // This is the best option available without realigning or changing the road speed.
        // Depending on speed+volume, it may be high or not. The user can manually upgrade to
        // SegregatedWithSpeedVolume.
        InfraType::Segregated
    }
}

/// This follows table 3.2 from https://www.transport.gov.scot/media/50323/cycling-by-design-update-2019-final-document-15-september-2021-1.pdf as closely as possible
pub fn get_level_of_service(
    infra_type: InfraType,
    speed: usize,
    traffic: usize,
    within_settlement: bool,
) -> LevelOfService {
    match infra_type {
        // "Mixed Traffic Street"
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

        // "Cycle Track at Carriageway Level". Note this means high LoS is impossible to achieve on
        // some roads.
        InfraType::Segregated => {
            if speed <= 30 {
                LevelOfService::High
            } else if speed <= 40 {
                LevelOfService::Medium
            } else if speed <= 50 && traffic < 1000 {
                LevelOfService::Medium
            } else {
                LevelOfService::Low
            }
        }

        // By definition, high
        InfraType::SegregatedWithSpeedVolume => LevelOfService::High,

        // "Detached or Remote Cycle Track"
        InfraType::OffRoad => LevelOfService::High,

        // Not in CbD. Just depends on urban/rural.
        // TODO Settlements are not quite the same as urban areas.
        InfraType::SharedFootway => {
            if within_settlement {
                LevelOfService::Low
            } else {
                LevelOfService::High
            }
        }

        // "Cycle Lane"
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
pub fn get_speed_mph(hwy: Highway, tags: &Tags) -> usize {
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
