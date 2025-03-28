import type {
  Feature,
  FeatureCollection,
  LineString,
  MultiPolygon,
  Point,
  Polygon,
} from "geojson";

export type Tier = "Primary" | "Secondary" | "LocalAccess" | "LongDistance";

export type InfraType =
  | "Segregated"
  | "SegregatedWithSpeedVolume"
  | "OffRoad"
  | "SharedFootway"
  | "CycleLane"
  | "MixedTraffic";
export let infraTypes: [string, string, string][] = [
  ["Segregated", "Segregated track", "#054d05"],
  ["SegregatedWithSpeedVolume", "Segregated + traffic measures", "#469237"],
  ["OffRoad", "Off-road cycleway", "#87d668"],
  ["SharedFootway", "Shared footway", "#ffbf00"],
  ["CycleLane", "Painted lane", "#7faedd"],
  ["MixedTraffic", "Mixed traffic", "#FF00FF"],
];

// Map the name to [label, color]
export let infraTypeMapping: { [name: string]: [string, string] } =
  Object.fromEntries(
    infraTypes.map(([name, label, color]) => [name, [label, color]]),
  );

export interface RouteGJ extends FeatureCollection {
  car_length: number;
  quiet_bike_length: number;
  direct_bike_length: number;
  straight_line_length: number;
  directions: Step[];
}

export interface Step {
  name?: string;
  length: number;
  way: string;
  infra_type: string;
  los: string;
}

export type EvaluateODOut = FeatureCollection<
  LineString,
  { count: number; infra_type: string; los: string }
> & {
  succeeded: number;
  failed: number;
  max_count: number;
} & ODStats;

export type WorstRoutes = [
  { x: number; y: number },
  { x: number; y: number },
][];

export interface ODStats {
  od_percents_infra_type: { [name: string]: number };
  od_percents_tier: { [name: string]: number };
  od_percents_los: { [name: string]: number };
  average_weighted_directness: number;
  worst_directness_routes: WorstRoutes;
}

export interface Stats {
  percent_reachable_schools: number;
  percent_reachable_gp_hospitals: number;
  percent_reachable_town_centres: number;
  percent_reachable_settlements: number;
  percent_reachable_greenspaces: number;
  percent_reachable_imd_population: number;
  percent_reachable_population: number;

  covered_high_demand: number;
  total_high_demand: number;
  covered_medium_demand: number;
  total_medium_demand: number;

  total_network_length: number;
  total_high_los_length: number;
  total_low_gradient_length: number;
  total_undeliverable_length: number;

  total_main_road_length: number;
  covered_main_road_length: number;

  density_network_in_settlements: number | null;
}

export type POIs = FeatureCollection<
  Point,
  {
    poi_kind: PoiKind;
    description: string;
    reachable: boolean;
    idx: number;
    sort: number;
  }
>;

export type TownCentres = FeatureCollection<
  MultiPolygon,
  { poi_kind: PoiKind; description: string; reachable: boolean; idx: number }
>;

export type Settlements = FeatureCollection<
  MultiPolygon,
  {
    poi_kind: PoiKind;
    description: string;
    reachable: boolean;
    idx: number;
  }
>;

export type Greenspaces = FeatureCollection<
  Point | MultiPolygon,
  {
    poi_kind: PoiKind;
    kind: "greenspace" | "access point";
    description?: string;
    reachable?: boolean;
    idx?: number;
    sort?: number;
    centroid?: [number, number];
  }
>;

export type DataZones = FeatureCollection<
  MultiPolygon,
  {
    id: string;
    imd_rank: number;
    imd_percentile: number;
    population: number;
    area_km2: number;
    reachable: boolean;
    density_quintile: number;
  }
>;

export type GridMeshDensity = FeatureCollection<
  Polygon,
  { routes: number; total: number }
>;

export type PrecalculatedDemand = FeatureCollection<
  LineString,
  { demand: number; covered: boolean; quintile: number }
> & {
  covered_quintile_sums: number[];
  total_quintile_sums: number[];
};

export interface Waypoint {
  point: [number, number];
  snapped: boolean;
}

// TODO Reconcile these two

export interface SetRouteInput {
  feature: Feature<LineString, RouteProps>;
  roads: [number, "Forwards" | "Backwards"][];

  name: string;
  notes: string;
  infra_type: string;
  override_infra_type: boolean;
  tier: Tier;
}

export interface RouteProps {
  waypoints: Waypoint[];
  id: number;
  roads: [number, "Forwards" | "Backwards"][];

  name: string;
  notes: string;
  infra_type: InfraType;
  override_infra_type: boolean;
  tier: Tier;
}

export type ConnectedComponents = FeatureCollection<
  LineString,
  { component: number }
> & {
  component_lengths: number[];
  component_bboxes: [number, number, number, number][];
};

export type AutosplitRoute = FeatureCollection<
  LineString,
  {
    kind: "new" | "overlap";
    length: number;
    infra_type?: string;
    fits: boolean;
    gradient_group: "<= 3%" | "3 - 5%" | "5 - 7%" | "7 - 10%" | "> 10%";
    los: string;
    tier: Tier;
  }
>;

export type PoiKind =
  | "schools"
  | "gp_hospitals"
  | "greenspaces"
  | "town_centres"
  | "settlements";

export interface StaticRoad {
  id: number;
  way: number;
  is_main_road: boolean;
  within_settlement: boolean;
  traffic: number;
  cn: Tier | null;
  speed: number;
  gradient: number;
  existing_infra: string | null;
  precalculated_demand: number;
  precalculated_demand_group: "high" | "medium" | "";
  street_space: "Segregated" | "nothing" | null;
}

export interface DynamicRoad {
  id: number;
  los: string;
  reachable: "network" | "severance" | "reachable" | "unreachable";

  // All or nothing
  current_route_id: number | null;
  current_route_name: string | null;
  current_infra: string | null;
  current_tier: Tier | null;
  current_infra_fits: boolean;
}
