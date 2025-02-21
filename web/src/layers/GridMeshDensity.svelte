<script lang="ts">
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { SequentialLegend } from "svelte-utils";
  import { makeRamp, Popup } from "svelte-utils/map";
  import { layerId, percent } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { GridMeshDensity } from "../types";
  import LayerControls from "./LayerControls.svelte";
  import { gridMeshDensity as show } from "./stores";

  let data: GridMeshDensity = {
    type: "FeatureCollection",
    features: [],
  };

  let colorScale = ["#d7191c", "#87d668", "#3a9120", "#054d05"];
  // Route length
  let limits = [0, 1_600, 3_200, 6_400];
  // Mesh density units
  let legendLimits = [">800m", "≤800m", "≤400m", "≤200m"];

  let resolution = 800;
  let xOffset = 0;
  let yOffset = 0;

  // Don't use lastUpdate pattern here, since there are other cache keys here
  async function recalc(resolution: number, xOffset: number, yOffset: number) {
    if ($backend) {
      data = await $backend.getGridMeshDensity(
        resolution,
        xOffset * resolution,
        yOffset * resolution,
      );
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc(resolution, xOffset, yOffset);
  }
</script>

<LayerControls name="Mesh density (grid)" bind:show={$show}>
  <SequentialLegend {colorScale} limits={legendLimits} />
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("mesh-density-grid")}
    manageHoverState
    paint={{
      "fill-color": makeRamp(["get", "routes"], limits, colorScale),
      "fill-opacity": hoverStateFilter(0.5, 0.8),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      <p>{props.routes.toFixed(1)} meters of routes inside</p>
      <p>{props.total.toFixed(1)} meters of all roads inside</p>
      <p>So that's {percent(props.routes, props.total)}</p>
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
