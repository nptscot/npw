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
export function layerId(layerId: string): LayerProps {
  return {
    id: layerId,
    beforeId: getBeforeId(layerId),
    eventsIfTopMost: true,
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

// All layer IDs used with layerId must be defined here, with later entries
// drawn on top.

// Helpful docs:
// https://docs.maptiler.com/schema/planet/
// https://cloud.maptiler.com/maps/streets-v2/
const layerZorder = [
  streets("Ferry line"),

  // Reference layers (areas)
  "all-population",
  "all-population-outline",
  "simd",
  "simd-outline",
  "mesh-density-grid",
  "mesh-density-grid-outline",

  // Reference layers (lines)
  "precalculated-rnet",
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

  // Reference layers (points or small polygons)
  "gp-hospitals",
  "schools",
  "settlements-fill",
  "settlements-outline-reachable",
  "settlements-outline-unreachable",
  "town-centres-fill",
  "town-centres-outline-reachable",
  "town-centres-outline-unreachable",
  "greenspaces-fill",
  "greenspaces-outline-reachable",
  "greenspaces-outline-unreachable",
  "greenspace-access-points",

  // Draw these on top of the respective objects, in case the path goes through
  // the middle of a polygon
  "debug-reachability-settlements",
  "debug-reachability-town_centres",
  "debug-reachability-pois",

  "fix-reachability-settlements",
  "fix-reachability-town_centres",
  "fix-reachability-pois",

  "block-reference-layers",

  // Edit mode
  "snapper-lines",
  "snapper-preview",
  "edit-route-sections",

  // Eval route mode
  "eval-route-breakdown",
  "eval-car-route",
  "eval-quiet-bike-route",

  "draw-rectangle-fill",
  "draw-rectangle-outline",

  streets("Road labels"),

  "fade-study-area",
  "block-everything",
];
