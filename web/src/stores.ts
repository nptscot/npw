import * as Comlink from "comlink";
import type { Map } from "maplibre-gl";
import { get, writable, type Writable } from "svelte/store";
import { getKey, setLocalStorage } from "./common/files";
import type { Stats, Tier, WorstRoutes } from "./types";
import type { Backend } from "./worker";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "main" }
  | { kind: "edit-route"; id: number | null }
  | { kind: "evaluate-route"; prevMode: Mode; browse: WorstRoutes }
  | { kind: "debug-network" };

export let boundaryName = writable("");
// This may be updated by App upfront
export let currentFilename = writable("untitled");
export let mode: Writable<Mode> = writable({ kind: "main" });
export let tier: Writable<Tier> = writable("Primary");
export let map: Writable<Map | null> = writable(null);

// When the state is modified, trigger to refresh various things
export let mutationCounter = writable(1);

// TODO Does this need to be a store?
export let backend: Writable<Comlink.Remote<Backend> | null> = writable(null);

export let routeA: Writable<{ lng: number; lat: number } | null> =
  writable(null);
export let routeB: Writable<{ lng: number; lat: number } | null> =
  writable(null);

export let colorRoutesBy: Writable<"infra_type" | "tier"> =
  writable("infra_type");

export let interactiveMapLayersEnabled = writable(true);

export let remoteStorage = writable(true);

// For now, the user manually recalculates this
export let stats: Writable<Stats | null> = writable(null);

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
  setLocalStorage(getKey(boundary, filename), state);
}

export function assetUrl(path: string): string {
  return get(remoteStorage)
    ? `https://assets.od2net.org/tmp_npt_editor/${path}`
    : path;
}
