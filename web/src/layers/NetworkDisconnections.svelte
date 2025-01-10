<script lang="ts">
  import type { ExpressionSpecification, MapMouseEvent } from "maplibre-gl";
  import {
    GeoJSON,
    LineLayer,
    MapEvents,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { constructMatchExpression } from "svelte-utils/map";
  import { layerId, roadLineWidth } from "../common";
  import {
    backend,
    map,
    mutationCounter,
    type ConnectedComponents,
  } from "../stores";
  import LayerControls from "./LayerControls.svelte";

  let show = false;

  let lastUpdate = 0;
  let data: ConnectedComponents = {
    type: "FeatureCollection",
    features: [],
    component_sizes: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getConnectedComponents();
      lastUpdate = $mutationCounter;
    }
  }

  $: if (show && $mutationCounter > 0) {
    recalc();
  }

  let colors = ["#1b9e77", "#d95f02", "#7570b3", "#e7298a", "#66a61e"];
  let colorByComponent = constructMatchExpression(
    ["to-string", ["%", ["get", "component"], colors.length]],
    Object.fromEntries(colors.map((color, i) => [i.toString(), color])),
    "black",
  ) as ExpressionSpecification;

  let showComponent: number | null = null;

  function lineColor(showComponent: number | null): ExpressionSpecification {
    if (showComponent == null) {
      return colorByComponent;
    }
    return [
      "case",
      ["==", ["get", "component"], showComponent],
      colorByComponent,
      "black",
    ];
  }

  function clickLine(e: CustomEvent<LayerClickInfo>) {
    showComponent = e.detail.features[0].properties!.component;
  }

  function onClick(e: CustomEvent<MapMouseEvent>) {
    // If we click off a line, clear things
    if (
      $map!.queryRenderedFeatures(e.detail.point, {
        layers: ["network-disconnections"],
      }).length == 0
    ) {
      showComponent = null;
    }
  }
</script>

<LayerControls name="Network disconnections" bind:show>
  <p>
    The network you create should usually all be connected as one component.
  </p>

  <p>Component sizes:</p>
  <ul>
    {#each data.component_sizes.slice(0, 5) as size, idx}
      <li style:color={colors[idx % colors.length]}>{size}</li>
    {/each}
    {#if data.component_sizes.length > 5}
            <p>({data.component_sizes.length} components total; only 5 largest shown)</p>
    {/if}
  </ul>
</LayerControls>

<MapEvents on:click={onClick} />

<GeoJSON {data} generateId>
  <LineLayer
    {...layerId("network-disconnections")}
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "line-width": roadLineWidth(4),
      "line-color": lineColor(showComponent),
      "line-opacity": 0.8,
    }}
    manageHoverState
    hoverCursor="pointer"
    on:click={clickLine}
  />
</GeoJSON>
