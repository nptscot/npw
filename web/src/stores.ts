import type { Map } from "maplibre-gl";
import { get, writable, type Writable } from "svelte/store";
import { type Backend } from "./worker";
import { JsRouteSnapper } from "route-snapper";
import type { FeatureCollection } from "geojson";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "main" }
  | { kind: "import-route" }
  | { kind: "edit-route"; id: number | null }
  | { kind: "evaluate-route" }
  | { kind: "debug-network" }
  | { kind: "debug-od" };

export let mode: Writable<Mode> = writable({ kind: "main" });
export let map: Writable<Map | null> = writable(null);

// TODO Does this need to be a store?
export let backend: Writable<Backend | null> = writable(null);
export let routeSnapper: Writable<JsRouteSnapper | null> = writable(null);
export let coherentNetwork: Writable<FeatureCollection> = writable({
  type: "FeatureCollection",
  features: [],
});
export let odZones: Writable<FeatureCollection> = writable({
  type: "FeatureCollection",
  features: [],
});
export let odPairs: Writable<[string, string, number][]> = writable([]);

export let routeA: Writable<{ lng: number; lat: number } | null> =
  writable(null);
export let routeB: Writable<{ lng: number; lat: number } | null> =
  writable(null);

export let infraTypes: [string, string, string][] = [
  ["Separated", "Separated cycle track", "#008000"],
  ["Roadside", "Roadside infrastructure", "#FFBF00"],
  ["CycleLane", "Cycle lane on carriageway", "#FF0000"],
  ["MixedTraffic", "Mixed traffic", "#EFD1C5"],
  ["Unknown", "Unknown", "blue"],
];

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

export function parseOD(raw: string): [string, string, number][] {
  let lines = raw.split("\n");
  lines.shift();
  let od = [] as [string, string, number][];
  for (let line of lines) {
    let tuple = line.split(",");
    od.push([tuple[0], tuple[1], parseInt(tuple[2])]);
  }
  return od;
}
