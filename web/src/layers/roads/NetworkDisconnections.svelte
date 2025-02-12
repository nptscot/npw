<script lang="ts">
  import type { ExpressionSpecification, MapMouseEvent } from "maplibre-gl";
  import {
    GeoJSON,
    LineLayer,
    MapEvents,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { constructMatchExpression } from "svelte-utils/map";
  import {
    layerId,
    Link,
    prettyPrintDistance,
    roadLineWidth,
  } from "../../common";
  import {
    backend,
    map,
    mutationCounter,
    referenceRoadStyle,
  } from "../../stores";
  import type { ConnectedComponents } from "../../types";

  $: show = $referenceRoadStyle == "disconnections";

  let lastUpdate = 0;
  let data: ConnectedComponents = {
    type: "FeatureCollection",
    features: [],
    component_lengths: [],
    component_bboxes: [],
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
    ["to-string", ["get", "component"]],
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

{#if $referenceRoadStyle == "disconnections"}
  <p>
    The network you create should usually all be connected as one component.
  </p>

  <p>Components:</p>
  <ul>
    {#each data.component_lengths.slice(0, 5) as length, idx}
      <li style:color={colors[idx]}>
        <Link on:click={() => $map?.fitBounds(data.component_bboxes[idx])}>
          {prettyPrintDistance(length)}
        </Link>
      </li>
    {/each}
    {#if data.component_lengths.length > 5}
      <p>
        ({data.component_lengths.length} components total; only 5 largest shown)
      </p>
    {/if}
  </ul>
{/if}

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
