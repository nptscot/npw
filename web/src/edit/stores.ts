import { writable, type Writable } from "svelte/store";
import type { Waypoint } from "../types";

export const waypoints: Writable<Waypoint[]> = writable([]);
