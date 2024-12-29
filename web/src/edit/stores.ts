import { RouteTool } from "route-snapper-ts";
import { writable, type Writable } from "svelte/store";

export interface Waypoint {
  point: [number, number];
  snapped: boolean;
}

export let routeTool: Writable<RouteTool | null> = writable(null);
export const waypoints: Writable<Waypoint[]> = writable([]);
