<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import { meshDensity as show } from "./stores";

  let data: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.meshDensity();
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }
</script>

<LayerControls name="Mesh density" bind:show={$show}>
  <p>TODO</p>
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("mesh-density")}
    manageHoverState
    paint={{
      "fill-color": "yellow",
      "fill-opacity": hoverStateFilter(0.5, 0.8),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />

  <LineLayer
    {...layerId("mesh-density-outline")}
    paint={{
      "line-color": "black",
      "line-width": 2,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
