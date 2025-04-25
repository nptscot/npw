import { get, writable, type Writable } from "svelte/store";
import { mode } from "../stores";
import type { Waypoint } from "../types";

export const waypoints: Writable<Waypoint[]> = writable([]);

export function canStopDrawing(): boolean {
  if (get(mode).kind == "edit-route" && get(waypoints).length > 0) {
    if (
      !window.confirm(
        "Are you sure you want to cancel and discard these changes?",
      )
    ) {
      return false;
    }
  }
  return true;
}
