import { map as mapStore } from "../stores";
import { get } from "svelte/store";

interface LayerProps {
  id: string;
  beforeId: string | undefined;
}

// Use this helper for every svelte-maplibre layer component. It sets the layer
// ID, beforeId (for z-ordering between layers), and defaults to only using the
// top-most layer for hovering/clicking.
export function layerId(layerId: string): LayerProps {
  return {
    id: layerId,
    beforeId: getBeforeId(layerId),
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
  "urban-areas",

  "main-mode",

  // Reference layers (lines)
  "cn-debug",
  "cn",
  "existing-infra-debug",
  "existing-infra",
  "traffic-debug",
  "traffic",
  "gradients",
  "cycling-flow-1",
  "cycling-flow-2",
  "cycling-flow-3",
  "los-debug",
  "los",
  "rnet",
  "reachability",
  "debug-reachability-settlements",
  "debug-reachability-town_centres",
  "debug-reachability-greenspaces",
  "debug-reachability-schools",
  "debug-reachability-gp_hospitals",

  // Reference layers (points or small polygons)
  "gp-hospitals",
  "schools",
  "settlements",
  "town-centres",
  "greenspaces",
  "greenspace-access-points",
  "major-junctions",

  // Edit mode
  "snapper-lines",
  "snapper-preview",
  "edit-current-routes",
  "edit-route-sections",

  // Special modes
  "eval-od-mode",
  "debug-mode",
  "mesh-density",
  "mesh-density-outline",
  "eval-current-routes",
  "eval-route-breakdown",
  "eval-car-route",

  streets("Road labels"),

  "fade-study-area",
];
