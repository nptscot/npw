<script lang="ts">
  import type { Feature, Point } from "geojson";
  import { GeoJSON, SymbolLayer, type LayerClickInfo } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { autosave, backend, mutationCounter } from "../stores";
  import type { Schools } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { localPOIs as show } from "./stores";
  import WarpToPOIs from "./WarpToPOIs.svelte";

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

  async function fixUnreachable(e: CustomEvent<LayerClickInfo>) {
    let input = await $backend!.fixUnreachablePOI(
      "schools",
      e.detail.features[0].properties!.idx,
    );
    await $backend!.setRoute(null, input);
    await autosave();
    hovered = null;
  }
</script>

<LayerControls name="POIs" bind:show={$show}>
  <WarpToPOIs />
</LayerControls>

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
    on:click={fixUnreachable}
  >
    <Popup openOn="hover" let:props>
      <div style="max-width: 30vw; max-height: 60vh; overflow: auto;">
        {props.name} is a {props.kind} school with {props.pupils} pupils. It {props.reachable
          ? "is"
          : "is not"} reachable. {#if !props.reachable}Click to add the black
          route to connect it to the network.{/if}
      </div>
    </Popup>
  </SymbolLayer>
</GeoJSON>

<DebugReachability kind="schools" {hovered} />
