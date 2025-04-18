import type { Map } from "maplibre-gl";
import { get, writable, type Writable } from "svelte/store";
import { getKey, setLocalStorage } from "./common/files";
import { disableLayersPerStage, enableLayersPerStage } from "./layers/stores";
import { currentPOI } from "./local_access/stores";
import type {
  ConnectedComponents,
  ODStats,
  SlowStats,
  Stats,
  Tier,
  WorstRoutes,
} from "./types";
import type { Backend } from "./worker_wrapper";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode =
  | { kind: "setup"; subpage: "explore" | "project-list" | "new-project" }
  | { kind: "overview" }
  | { kind: "main" }
  | { kind: "edit-route"; id: number | null }
  | { kind: "evaluate-journey"; browse: WorstRoutes }
  | { kind: "bulk-edit" }
  | { kind: "export" };

export let boundaryName = writable("");
// When this is blank, changes aren't saved.
export let currentFilename = writable("");
export let mode: Writable<Mode> = writable({
  kind: "setup",
  subpage: "explore",
});
export let currentStage: Writable<Tier | "assessment"> = writable("Primary");
export let map: Writable<Map | null> = writable(null);
export let zoom: Writable<number | undefined> = writable(undefined);

// When the state is modified, trigger to refresh various things
export let mutationCounter = writable(1);

// TODO Does this need to be a store?
export let backend: Writable<Backend | null> = writable(null);
export let loadingSpinners = writable(0);

export let routeA: Writable<{ lng: number; lat: number } | null> =
  writable(null);
export let routeB: Writable<{ lng: number; lat: number } | null> =
  writable(null);

export type EditsRoadStyle =
  | "off"
  | "edits_infra"
  | "edits_tier"
  | "edits_deliverability"
  | "edits_los";

export type BackgroundLayer =
  | "off"
  | "cn"
  | "existing_infra"
  | "traffic"
  | "gradient"
  | "street_space"
  | "speed"
  | "attractive"
  | "los"
  | "reachability"
  | "disconnections"
  | "precalculated_rnet"
  | "calculated_rnet"
  | "population"
  | "deprived";

export let editsRoadStyle: Writable<EditsRoadStyle> = writable("edits_tier");
export let backgroundLayer: Writable<BackgroundLayer> = writable("off");

export let editModeBreakdown: Writable<
  "infra_type" | "gradient" | "deliverability" | "los" | "tier"
> = writable("infra_type");

export let interactiveMapLayersEnabled = writable(true);

export let remoteStorage = writable(true);
export let devMode = writable(import.meta.env.MODE == "development");

export let stats: Writable<Stats | null> = writable(null);
export let slowStats: Writable<SlowStats | null> = writable(null);
export let lastUpdateSlowStats = writable(0);

export let odStats: Writable<ODStats | null> = writable(null);
export let lastUpdateOD = writable(0);

export let connectedComponents: Writable<ConnectedComponents> = writable({
  type: "FeatureCollection",
  features: [],
  component_lengths: [],
  component_bboxes: [],
});

export async function autosave() {
  mutationCounter.update((x) => {
    return x + 1;
  });

  let filename = get(currentFilename);
  if (!filename) {
    console.error(`Autosave called without currentFilename set`);
    return;
  }

  let backendValue = get(backend);
  let boundary = get(boundaryName);
  if (!backendValue || !boundary) {
    return;
  }
  let state = await backendValue.getAllRoutes();
  setLocalStorage(getKey(boundary, filename), JSON.stringify(state));

  // TODO Temporary debugging
  /*let total = 0;
  for (let route of Object.values(state.features)) {
    total += (route as any).properties.waypoints.length;
  }
  console.log(`Autosaving. ${total} waypoints in all routes`);*/
}

// Updates the URL and enters the main state
export function setCurrentFile(name: string) {
  currentFilename.set(name);
  mutationCounter.update((x) => {
    return x + 1;
  });
  mode.set({ kind: "overview" });

  let url = new URL(window.location.href);
  url.searchParams.set("file", name);
  window.history.replaceState(null, "", url.toString());
}

export function assetUrl(path: string): string {
  let dir =
    import.meta.env.BASE_URL == "/npw/demo" ? "demo_npw" : "tmp_npt_editor";
  return get(remoteStorage) ? `https://assets.od2net.org/${dir}/${path}` : path;
}

// TODO Might work better as onDestroy of the components?
export function exitCurrentStage() {
  let oldStage = get(currentStage);

  // Disable old layers
  for (let show of disableLayersPerStage[oldStage]) {
    show.set(false);
  }
  if (oldStage == "assessment") {
    backgroundLayer.set("off");
  }
  if (oldStage == "LocalAccess") {
    currentPOI.set(null);
  }
}

export function changeStage(rawNewStage: string) {
  exitCurrentStage();

  mode.set({ kind: "main" });

  // Workaround TS
  let newStage = rawNewStage as Tier | "assessment";
  currentStage.set(newStage);

  // Show new layers
  for (let show of enableLayersPerStage[newStage]) {
    show.set(true);
  }
}
