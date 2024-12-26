import type {
  Feature,
  FeatureCollection,
  LineString,
  MultiPolygon,
  Point,
  Polygon,
} from "geojson";
import type { Map } from "maplibre-gl";
import { get, writable, type Writable } from "svelte/store";
import { getKey } from "./common/files";
import { type Backend } from "./worker";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "main" }
  | { kind: "edit-route"; id: number | null }
  | { kind: "evaluate-route"; prevMode: Mode; browse: WorstRoutes }
  | { kind: "evaluate-od" }
  | { kind: "debug-network" };
export type Tier = "Primary" | "Secondary" | "LocalAccess" | "LongDistance";

export let boundaryName = writable("");
// This may be updated by App upfront
export let currentFilename = writable("untitled");
export let mode: Writable<Mode> = writable({ kind: "main" });
export let tier: Writable<Tier> = writable("Primary");
export let map: Writable<Map | null> = writable(null);

// When the state is modified, trigger to refresh various things
export let mutationCounter = writable(1);

// TODO Does this need to be a store?
export let backend: Writable<Backend | null> = writable(null);

export let routeA: Writable<{ lng: number; lat: number } | null> =
  writable(null);
export let routeB: Writable<{ lng: number; lat: number } | null> =
  writable(null);

export let colorRoutesBy: Writable<"infra_type" | "tier"> =
  writable("infra_type");

export let interactiveMapLayersEnabled = writable(true);

export let infraTypes: [string, string, string][] = [
  ["SegregatedWide", "Segregated Track (wide)", "#054d05"],
  ["OffRoad", "Off Road Cycleway", "#3a9120"],
  ["SegregatedNarrow", "Segregated Track (narrow)", "#87d668"],
  ["SharedFootway", "Shared Footway", "#ffbf00"],
  ["CycleLane", "Painted Cycle Lane", "#FF0000"],
  ["MixedTraffic", "Mixed traffic", "#EFD1C5"],
  ["Unknown", "Unknown", "blue"],
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

export async function autosave() {
  mutationCounter.update((x) => {
    return x + 1;
  });

  let backendValue = get(backend);
  let boundary = get(boundaryName);
  if (!backendValue || !boundary) {
    return;
  }
  let filename = get(currentFilename);
  let state = await backendValue.toSavefile();
  window.localStorage.setItem(getKey(boundary, filename), state);
}

export let remoteStorage = writable(true);

export function assetUrl(path: string): string {
  return get(remoteStorage)
    ? `https://assets.od2net.org/tmp_npt_editor/${path}`
    : path;
}

export type EvaluateODOut = FeatureCollection & {
  succeeded: number;
  failed: number;
  max_count: number;
  percent_off_network: number;
  percent_on_network: { [name: string]: number };
};

export type WorstRoutes = [
  { x: number; y: number },
  { x: number; y: number },
][];

export interface Stats {
  od_percents_infra_type: { [name: string]: number };
  od_percents_los: { [name: string]: number };
  average_weighted_directness: number;
  percent_reachable_schools: number;
  percent_reachable_gp_hospitals: number;
  percent_reachable_town_centres: number;
  percent_reachable_settlements: number;
  percent_reachable_greenspaces: number;
  percent_reachable_imd_population: number;
  percent_reachable_population: number;
  worst_directness_routes: WorstRoutes;
  covered_flow_quintile_sums: number[];
  total_flow_quintile_sums: number[];
}
// For now, the user manually recalculates this
export let stats: Writable<Stats | null> = writable(null);

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
