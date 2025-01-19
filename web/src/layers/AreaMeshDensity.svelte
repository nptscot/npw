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
  import { backend, mutationCounter } from "../stores";
  import type { AreaMeshDensity } from "../types";
  import LayerControls from "./LayerControls.svelte";
  import { areaMeshDensity as show } from "./stores";

  let lastUpdate = 0;
  let data: AreaMeshDensity = {
    type: "FeatureCollection",
    features: [],
  };

  let colorScale = ["#1a9641", "#a6d96a", "#fdae61", "#d7191c"];
  let limits = [0, 0.04, 0.2, 0.5, 1000];

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getAreaMeshDensity();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }
</script>

<LayerControls name="Mesh density (area)" bind:show={$show}>
  <SequentialLegend {colorScale} {limits} />
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("mesh-density-area")}
    manageHoverState
    paint={{
      "fill-color": makeRamp(["get", "area"], limits, colorScale),
      "fill-opacity": hoverStateFilter(0.5, 0.8),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.area.toFixed(1)} km
      <sup>2</sup>
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("mesh-density-area-outline")}
    paint={{
      "line-color": "black",
      "line-width": 2,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
