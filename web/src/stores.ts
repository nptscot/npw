import type { Map } from "maplibre-gl";
import { get, writable, type Writable } from "svelte/store";
import { type Backend } from "./worker";
import { JsRouteSnapper } from "route-snapper";
import type { FeatureCollection } from "geojson";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "main" }
  | { kind: "edit-route"; id: number | null }
  | { kind: "evaluate-route" }
  | { kind: "evaluate-od" }
  | { kind: "debug-network" }
  | { kind: "debug-mesh-density" };

export let mode: Writable<Mode> = writable({ kind: "main" });
export let map: Writable<Map | null> = writable(null);

// TODO Does this need to be a store?
export let backend: Writable<Backend | null> = writable(null);
export let routeSnapper: Writable<JsRouteSnapper | null> = writable(null);

export let routeA: Writable<{ lng: number; lat: number } | null> =
  writable(null);
export let routeB: Writable<{ lng: number; lat: number } | null> =
  writable(null);

export let infraTypes: [string, string, string][] = [
  ["SegregatedWide", "Segregated Track (wide)", "#054d05"],
  ["OffRoad", "Off Road Cycleway", "#3a9120"],
  ["SegregatedNarrow", "Segregated Track (narrow)", "#87d668"],
  ["SharedFootway", "Shared Footway", "#ffbf00"],
  ["CycleLane", "Painted Cycle Lane", "#FF0000"],
  ["MixedTraffic", "Mixed traffic", "#EFD1C5"],
  ["Unknown", "Unknown", "blue"],
];

export let infraTypeMapping: { [name: string]: [string, string] } =
  Object.fromEntries(
    infraTypes.map(([name, label, color]) => [name, [label, color]]),
  );

export interface RouteGJ extends FeatureCollection {
  direct_length: number;
  route_length: number;
  directions: Step[];
}

export interface Step {
  name?: string;
  length: number;
  way: string;
  infra_type: string;
}

export async function autosave() {
  let backendValue = get(backend);
  if (!backendValue) {
    return;
  }
  let state = await backendValue.toSavefile();
  window.localStorage.setItem("tmp-npt-editor", state);
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

export interface Stats {
  od_percents: { [name: string]: number };
  average_weighted_directness: number;
}
// For now, the user manually recalculates this
export let stats: Writable<Stats | null> = writable(null);
