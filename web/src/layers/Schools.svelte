<script lang="ts">
  import type { Feature, Point } from "geojson";
  import { GeoJSON, SymbolLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { Schools } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { localPOIs as show } from "./stores";

  let lastUpdate = 0;
  let data: Schools = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<Point, { reachable: boolean; idx: number }> | null;

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getSchools();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }
</script>

<LayerControls name="POIs" bind:show={$show} empty />

<GeoJSON {data} generateId>
  <SymbolLayer
    {...layerId("schools")}
    manageHoverState
    layout={{
      visibility: $show ? "visible" : "none",
      "icon-allow-overlap": true,
      "icon-size": ["interpolate", ["linear"], ["zoom"], 10, 0.1, 12, 1.0],
      "icon-image": [
        "case",
        ["get", "reachable"],
        "school_reachable",
        "school_unreachable",
      ],
    }}
    bind:hovered
    hoverCursor="pointer"
  >
    <Popup openOn="click" let:props>
      {props.name} is a {props.kind} school with {props.pupils} pupils. It {props.reachable
        ? "is"
        : "is not"} reachable.
    </Popup>
  </SymbolLayer>
</GeoJSON>

<DebugReachability kind="schools" {hovered} />
