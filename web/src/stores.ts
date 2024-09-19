import type { Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";
import { type Backend } from "./worker";
import { RouteTool } from "route-snapper-ts";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "main" }
  | { kind: "sketch-route"; id: number | null }
  | { kind: "route-details"; id: number }
  | { kind: "debug" };

export let mode: Writable<Mode> = writable({ kind: "main" });
export let map: Writable<Map | null> = writable(null);

// TODO Does this need to be a store?
export let backend: Writable<Backend | null> = writable(null);
export let routeTool: Writable<RouteTool | null> = writable(null);

export let infraTypes: [string, string, string][] = [
  ["Separated", "Separated cycle-track", "green"],
  ["Roadside", "Roadside (stepped)", "blue"],
  ["CycleLane", "Cycle lane", "yellow"],
  ["MixedTraffic", "Mixed traffic", "purple"],
  ["Unknown", "Unknown (but not mixed traffic)", "cyan"],
];
