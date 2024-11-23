<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { backend, type PrecalculatedFlows } from "../stores";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { highRouteCoverage as show } from "./stores";
  import { lineWidthForDemand, lineColorForDemand } from "../utils";

  let onlyCovered = false;

  let data: PrecalculatedFlows = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.renderPrecalculatedFlows();
    }
  }

  $: if ($show && data.features.length == 0) {
    recalc();
  }
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={$show} />
    High NPT route coverage
  </label>

  {#if $show}
    <button on:click={recalc}>Recalculate</button>
    <label>
      <input type="checkbox" bind:checked={onlyCovered} />
      Only show routes covered by current edits
    </label>
  {/if}
</LayerControls>

<GeoJSON {data}>
  <LineLayer
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    filter={onlyCovered ? ["get", "covered"] : undefined}
    paint={{
      "line-width": lineWidthForDemand("flow"),
      "line-color": lineColorForDemand("flow"),
    }}
  >
    <Popup openOn="hover" let:props>
      Flow {props.flow.toLocaleString()}, covered {props.covered}
    </Popup>
  </LineLayer>
</GeoJSON>
