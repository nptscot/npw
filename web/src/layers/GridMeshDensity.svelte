<script lang="ts">
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { SequentialLegend } from "svelte-utils";
  import { makeRamp, Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend, mutationCounter, type GridMeshDensity } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import { gridMeshDensity as show } from "./stores";

  let data: GridMeshDensity = {
    type: "FeatureCollection",
    features: [],
  };

  let colorScale = ["#d7191c", "#fdae61", "#a6d96a", "#1a9641"];
  let limits = [0, 100, 200, 300, 1000];

  async function recalc() {
    if ($backend) {
      data = await $backend.getGridMeshDensity();
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }
</script>

<LayerControls name="Mesh density (grid)" bind:show={$show}>
  <SequentialLegend {colorScale} {limits} />
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("mesh-density-grid")}
    manageHoverState
    paint={{
      "fill-color": makeRamp(["get", "length"], limits, colorScale),
      "fill-opacity": hoverStateFilter(0.5, 0.8),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.length.toFixed(1)} meters of routes inside
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("mesh-density-grid-outline")}
    paint={{
      "line-color": "black",
      "line-width": 1,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
