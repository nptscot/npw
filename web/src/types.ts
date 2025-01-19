import type {
  Feature,
  FeatureCollection,
  LineString,
  MultiPolygon,
  Point,
  Polygon,
} from "geojson";

export type Tier = "Primary" | "Secondary" | "LocalAccess" | "LongDistance";

export let infraTypes: [string, string, string][] = [
  ["SegregatedWide", "Segregated Track (wide)", "#054d05"],
  ["OffRoad", "Off Road Cycleway", "#3a9120"],
  ["SegregatedNarrow", "Segregated Track (narrow)", "#87d668"],
  ["SharedFootway", "Shared Footway", "#ffbf00"],
  ["CycleLane", "Painted Cycle Lane", "#FF0000"],
  ["MixedTraffic", "Mixed traffic", "#FF00FF"],
];

// Map the name to [label, color]
export let infraTypeMapping: { [name: string]: [string, string] } =
  Object.fromEntries(
    infraTypes.map(([name, label, color]) => [name, [label, color]]),
  );

export interface RouteGJ extends FeatureCollection {
  direct_length: number;
  car_length: number;
  direct_bike_length: number;
  route_length: number;
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

export interface Stats extends ODStats {
  percent_reachable_schools: number;
  percent_reachable_gp_hospitals: number;
  percent_reachable_town_centres: number;
  percent_reachable_settlements: number;
  percent_reachable_greenspaces: number;
  percent_reachable_imd_population: number;
  percent_reachable_population: number;
  covered_flow_quintile_sums: number[];
  total_flow_quintile_sums: number[];
}

export type Schools = FeatureCollection<
  Point,
  {
    kind: string;
    name: string;
    pupils: number;
    reachable: boolean;
    idx: number;
  }
>;

export type GPHospitals = FeatureCollection<
  Point,
  { kind: string; name: string; reachable: boolean; idx: number }
>;

export type TownCentres = FeatureCollection<
  MultiPolygon,
  { name?: string; reachable: boolean; idx: number }
>;

export type Settlements = FeatureCollection<
  MultiPolygon,
  { name?: string; population: number; reachable: boolean; idx: number }
>;

export type Greenspaces = FeatureCollection<
  Point | MultiPolygon,
  {
    kind: "greenspace" | "access point";
    name?: string;
    reachable?: boolean;
    idx?: number;
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

export type AreaMeshDensity = FeatureCollection<Polygon, { area: number }>;
export type GridMeshDensity = FeatureCollection<Polygon, { length: number }>;

export type PrecalculatedFlows = FeatureCollection<
  LineString,
  { flow: number; covered: boolean; quintile: number }
> & {
  covered_quintile_sums: number[];
  total_quintile_sums: number[];
};

export type RouteNode = { snapped: number } | { free: [number, number] };

// TODO Reconcile these two

export interface SetRouteInput {
  feature: Feature<LineString, RouteProps>;
  name: string;
  notes: string;
  full_path: RouteNode[];
  infra_type: string;
  tier: Tier;
}

export interface RouteProps {
  name: string;
  notes: string;
  full_path: RouteNode[];
  waypoints: any[];
  infra_type: string;
  tier: Tier;
}

export type ConnectedComponents = FeatureCollection<
  LineString,
  { component: number }
> & {
  component_sizes: number[];
};
