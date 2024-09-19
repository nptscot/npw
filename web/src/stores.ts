import type { Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";
import { type Backend } from "./worker";
import { RouteTool } from "route-snapper-ts";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "main" }
  | { kind: "sketch-route"; id: number | null }
  | { kind: "debug" };

export let mode: Writable<Mode> = writable({ kind: "main" });
export let map: Writable<Map | null> = writable(null);

// TODO Does this need to be a store?
export let backend: Writable<Backend | null> = writable(null);
export let routeTool: Writable<RouteTool | null> = writable(null);
