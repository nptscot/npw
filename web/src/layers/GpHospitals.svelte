<script lang="ts">
  import type { Feature, Point } from "geojson";
  import { GeoJSON, SymbolLayer, type LayerClickInfo } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { autosave, backend, mutationCounter } from "../stores";
  import type { GPHospitals } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import { localPOIs as show } from "./stores";

  let lastUpdate = 0;
  let data: GPHospitals = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<Point, { reachable: boolean; idx: number }> | null;

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getGpHospitals();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }

  async function fixUnreachable(e: CustomEvent<LayerClickInfo>) {
    let input = await $backend!.fixUnreachablePOI(
      "gp_hospitals",
      e.detail.features[0].properties!.idx,
    );
    await $backend!.setRoute(null, input);
    await autosave();
    hovered = null;
  }
</script>

<GeoJSON {data} generateId>
  <SymbolLayer
    {...layerId("gp-hospitals")}
    manageHoverState
    layout={{
      visibility: $show ? "visible" : "none",
      "icon-allow-overlap": true,
      "icon-size": ["interpolate", ["linear"], ["zoom"], 10, 0.1, 12, 1.0],
      "icon-image": [
        "case",
        ["get", "reachable"],
        "hospital_reachable",
        "hospital_unreachable",
      ],
    }}
    bind:hovered
    hoverCursor="pointer"
    on:click={fixUnreachable}
  >
    <Popup openOn="hover" let:props>
      <div style="max-width: 30vw; max-height: 60vh; overflow: auto;">
        {props.name} is a {props.kind}. It {props.reachable ? "is" : "is not"} reachable.
        {#if !props.reachable}Click to add the black route to connect it to the
          network.{/if}
      </div>
    </Popup>
  </SymbolLayer>
</GeoJSON>

<DebugReachability kind="gp_hospitals" {hovered} />
