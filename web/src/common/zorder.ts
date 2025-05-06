import { get } from "svelte/store";
import { map as mapStore } from "../stores";

interface LayerProps {
  id: string;
  beforeId: string | undefined;
  eventsIfTopMost: boolean;
}

// Use this helper for every svelte-maplibre layer component. It sets the layer
// ID, beforeId (for z-ordering between layers), and defaults to only using the
// top-most layer for hovering/clicking.
export function layerId(
  layerId: string,
  eventsIfTopMost: boolean = true,
): LayerProps {
  return {
    id: layerId,
    beforeId: getBeforeId(layerId),
    eventsIfTopMost,
  };
}

// Calculates the beforeId for adding a layer. Due to hot-module reloading and
// Svelte component initialization order being unpredictable, callers might add
// layers in any order. Use beforeId to guarantee the layers wind up in an
// explicitly defined order.
function getBeforeId(layerId: string): string | undefined {
  let map = get(mapStore);
  if (!map) {
    console.warn(
      `getBeforeId(${layerId}) called before map is ready. Z-ordering may be incorrect.`,
    );
    return undefined;
  }

  // Find the last layer currently in the map that should be on top of this new
  // layer.
  let beforeId = undefined;
  let found = false;
  for (let i = layerZorder.length - 1; i >= 0; i--) {
    let id = layerZorder[i];
    if (id == layerId) {
      found = true;
      break;
    }
    if (map.getLayer(id)) {
      beforeId = id;
    }
  }
  // When adding a new layer somewhere, force the programmer to decide where it
  // should be z-ordered.
  if (!found) {
    throw new Error(`Layer ID ${layerId} not defined in layerZorder`);
  }
  // If beforeId isn't set, we'll add the layer on top of everything else.
  return beforeId;
}

// Dummy functions just used for documentation below.
let streets = (x: string) => x;
let datavizLight = (x: string) => x;

// All layer IDs used with layerId must be defined here, with later entries
// drawn on top.

// Helpful docs:
// https://docs.maptiler.com/schema/planet/
// https://cloud.maptiler.com/maps/streets-v2/
const layerZorder = [
  streets("Ferry line"),
  datavizLight("Railway dash"),

  // Reference layers (areas)
  "population",
  "uncovered-population-fill",
  "uncovered-population-outline",
  "mesh-density-grid",
  "mesh-density-grid-outline",

  // Reference layers (points or polygons)
  "railway-stations", // TODO Need to cover up the basemap one sometimes
  "schools",
  "gp-hospitals",
  "greenspaces-fill",
  "greenspaces-outline",
  "current-poi",
  "settlements-fill",
  "settlements-outline",
  "town-centres-fill",
  "town-centres-outline",
  "town-centres-points",
  "greenspace-access-points",
  "major-junctions",

  // Roads
  "precalculated-rnet-debug",
  "uncovered-cycling-demands",
  "uncovered-main-roads",
  "edits-roads",
  "reference-roads",
  "cn-debug",
  "existing-infra-debug",
  "street_space-debug",
  "traffic-debug",
  "speed_limit-debug",
  "los-debug",
  "calculated-rnet",
  "network-disconnections",

  // Draw these on top of the respective objects, in case the path goes through
  // the middle of a polygon
  "debug-reachability-pois",
  "fix-reachability-pois",

  "block-reference-layers",

  // Edit mode
  "edit-route-sections",
  "snapper-preview",
  "snapper-current-snapped-waypoint",
  "snapper-debug-cursor",

  // Eval route mode
  "eval-route-breakdown",
  "eval-quiet-bike-route",

  "directness-network",

  "draw-rectangle-fill",
  "draw-rectangle-outline",

  streets(datavizLight("Road labels")),

  "fade-study-area-inside",
  "fade-study-area-outside",
  "fade-study-area-entire",
  "study-area-outline",
  "block-everything",
];
