<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { PrecalculatedFlows } from "../types";
  import { lineWidthForDemand, percent } from "../utils";
  import LayerControls from "./LayerControls.svelte";

  export let show: boolean;
  export let quintile: number;
  export let label: string;

  let lastUpdate = 0;
  let data: PrecalculatedFlows = {
    type: "FeatureCollection",
    features: [],
    covered_quintile_sums: [],
    total_quintile_sums: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.renderPrecalculatedFlows();
      lastUpdate = $mutationCounter;
    }
  }

  $: if (show && $mutationCounter > 0) {
    recalc();
  }
</script>

<LayerControls name={label + " cycling flow"} bind:show>
  {#if data.total_quintile_sums.length > 0}
    <span class="legend" />

    <p>
      {percent(
        data.covered_quintile_sums[quintile - 1],
        data.total_quintile_sums[quintile - 1],
      )} of quintile {quintile} flows covered
    </p>
  {/if}
</LayerControls>

<GeoJSON {data}>
  <LineLayer
    {...layerId("cycling-flow-" + quintile)}
    layout={{
      visibility: show ? "visible" : "none",
    }}
    filter={["all", ["!", ["get", "covered"]], ["==", ["get", "quintile"], quintile]]}
    paint={{
      "line-width": lineWidthForDemand("flow"),
      "line-color": "grey",
    }}
  >
    <Popup openOn="hover" let:props>
      Flow {props.flow.toLocaleString()} is in quintile {props.quintile},
      covered {props.covered}
    </Popup>
  </LineLayer>
</GeoJSON>

<style>
  .legend {
    float: left;
    height: 16px;
    width: 30px;
    margin-right: 5px;
    background-color: grey;
  }
</style>
