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
  import {
    backend,
    mutationCounter,
    type AreaMeshDensity,
    type GridMeshDensity,
  } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import { meshDensity as show } from "./stores";

  let method: "area" | "grid" = "grid";
  let data: AreaMeshDensity | GridMeshDensity = {
    type: "FeatureCollection",
    features: [],
  };

  // For area
  let colorScale = ["#1a9641", "#a6d96a", "#fdae61", "#d7191c"];
  let limits = [0, 0.04, 0.2, 0.5, 1000];

  async function recalc() {
    if ($backend) {
      if (method == "area") {
        data = await $backend.getAreaMeshDensity();
      } else {
        data = await $backend.getGridMeshDensity();
      }
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }
</script>

<LayerControls name="Mesh density" bind:show={$show}>
  <fieldset>
    <legend>Method:</legend>
    <label>
      <input type="radio" value="grid" bind:group={method} />
      Grid
    </label>
    <label>
      <input type="radio" value="area" bind:group={method} />
      Area bound
    </label>
  </fieldset>

  {#if method == "area"}
    <SequentialLegend {colorScale} {limits} />
  {/if}
</LayerControls>

{#if method == "area"}
  <GeoJSON {data} generateId>
    <FillLayer
      {...layerId("mesh-density")}
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
{:else if method == "grid"}
  <GeoJSON {data} generateId>
    <FillLayer
      {...layerId("mesh-density")}
      manageHoverState
      paint={{
        "fill-color": "grey",
        "fill-opacity": hoverStateFilter(0.1, 0.8),
      }}
      layout={{
        visibility: $show ? "visible" : "none",
      }}
    >
      <Popup openOn="hover" let:props>
        {props.length.toFixed(1)} km of routes inside
      </Popup>
    </FillLayer>

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
{/if}
