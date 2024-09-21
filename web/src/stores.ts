import type { Map } from "maplibre-gl";
import { get, writable, type Writable } from "svelte/store";
import { type Backend } from "./worker";
import { RouteTool } from "route-snapper-ts";
import type { FeatureCollection } from "geojson";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "main" }
  | { kind: "import-route" }
  | { kind: "sketch-route"; id: number | null }
  | { kind: "route-details"; id: number }
  | { kind: "evaluate-route" }
  | { kind: "debug" };

export let mode: Writable<Mode> = writable({ kind: "main" });
export let map: Writable<Map | null> = writable(null);

// TODO Does this need to be a store?
export let backend: Writable<Backend | null> = writable(null);
export let routeTool: Writable<RouteTool | null> = writable(null);
export let coherentNetwork: Writable<FeatureCollection> = writable({
  type: "FeatureCollection",
  features: [],
});

export let routeA: Writable<{ lng: number; lat: number } | null> =
  writable(null);
export let routeB: Writable<{ lng: number; lat: number } | null> =
  writable(null);

export let infraTypes: [string, string, string][] = [
  ["Separated", "Separated cycle-track", "green"],
  ["Roadside", "Roadside (stepped)", "blue"],
  ["CycleLane", "Cycle lane", "yellow"],
  ["MixedTraffic", "Mixed traffic", "purple"],
  ["Unknown", "Unknown (but not mixed traffic)", "cyan"],
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
