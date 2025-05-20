use enum_map::Enum;
use graph::RoadID;
use serde::{Deserialize, Serialize};

use crate::{InfraType, MapModel};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum, Serialize)]
pub enum LevelOfService {
    High,
    Medium,
    Low,
    ShouldNotBeUsed,
}

impl LevelOfService {
    /// A multiplier to penalize traveling along a road with this LoS. These values are made up,
    /// but the quiet routes seem reasonable.
    pub fn penalty(self) -> f64 {
        match self {
            LevelOfService::High => 1.0,
            LevelOfService::Medium => 1.5,
            LevelOfService::Low => 3.0,
            LevelOfService::ShouldNotBeUsed => 5.0,
        }
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
pub enum TrafficVolume {
    UpTo1000,
    UpTo2000,
    UpTo4000,
    Over4000,
}

impl MapModel {
    pub fn calculate_level_of_service(&self, r: RoadID) -> LevelOfService {
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
    traffic: TrafficVolume,
    within_settlement: bool,
) -> LevelOfService {
    match infra_type {
        // "Mixed Traffic Street"
        InfraType::MixedTraffic => {
            if speed <= 20 {
                if traffic <= TrafficVolume::UpTo2000 {
                    LevelOfService::High
                } else if traffic <= TrafficVolume::UpTo4000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else if speed <= 30 {
                if traffic <= TrafficVolume::UpTo1000 {
                    LevelOfService::High
                } else if traffic <= TrafficVolume::UpTo2000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else if speed <= 40 {
                if traffic <= TrafficVolume::UpTo1000 {
                    LevelOfService::Medium
                } else if traffic <= TrafficVolume::UpTo2000 {
                    LevelOfService::Low
                } else {
                    LevelOfService::ShouldNotBeUsed
                }
            } else if speed <= 60 {
                // Both 50 and 60mph cases
                if traffic <= TrafficVolume::UpTo1000 {
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
            } else if speed <= 50 && traffic <= TrafficVolume::UpTo1000 {
                LevelOfService::Medium
            } else {
                LevelOfService::Low
            }
        }

        // By definition, high
        InfraType::SegregatedWithSpeedVolume => LevelOfService::High,
        InfraType::MixedTrafficWithSpeedVolume => LevelOfService::High,

        // "Detached or Remote Cycle Track"
        InfraType::OffRoad => LevelOfService::High,

        InfraType::SharedFootway => {
            // Always low within a settlement
            if within_settlement {
                LevelOfService::Low
            } else {
                // Use "Stepped or Footway Level Cycle Track" CbD rules. The speed and volume of
                // motorized roads are copied onto this road, with plenty of map-matching caveats
                if speed <= 20 {
                    LevelOfService::High
                } else if speed <= 30 {
                    if traffic <= TrafficVolume::UpTo4000 {
                        LevelOfService::High
                    } else {
                        LevelOfService::Medium
                    }
                } else if speed <= 40 {
                    LevelOfService::Medium
                } else if speed <= 50 {
                    if traffic == TrafficVolume::UpTo1000 {
                        LevelOfService::Medium
                    } else {
                        LevelOfService::Low
                    }
                } else {
                    LevelOfService::Low
                }
            }
        }

        // "Cycle Lane"
        InfraType::CycleLane => {
            if speed <= 20 {
                if traffic <= TrafficVolume::UpTo4000 {
                    LevelOfService::High
                } else {
                    LevelOfService::Medium
                }
            } else if speed <= 30 {
                if traffic <= TrafficVolume::UpTo1000 {
                    LevelOfService::High
                } else if traffic <= TrafficVolume::UpTo4000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else if speed <= 40 {
                if traffic <= TrafficVolume::UpTo1000 {
                    LevelOfService::Medium
                } else {
                    LevelOfService::Low
                }
            } else if speed <= 50 {
                LevelOfService::Low
            } else if speed <= 60 {
                if traffic <= TrafficVolume::UpTo1000 {
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
