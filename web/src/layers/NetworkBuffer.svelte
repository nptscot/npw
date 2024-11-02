<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { backend } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import type { FeatureCollection } from "geojson";

  // TODO Does this belong as a layer like this, or a debug mode, in the short term?

  let show = false;

  let data: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    data = await $backend!.getNetworkBuffer();
  }

  $: if (show && data.features.length == 0) {
    recalc();
  }
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Network buffer
  </label>

  {#if show}
    <button on:click={recalc}>Recalculate</button>
  {/if}
</LayerControls>

<GeoJSON {data} generateId>
  <LineLayer
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "line-width": hoverStateFilter(5, 7),
      "line-color": "black",
      "line-opacity": 0.8,
    }}
    manageHoverState
  />
</GeoJSON>
