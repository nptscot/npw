<script lang="ts">
  import type { ExpressionSpecification, MapMouseEvent } from "maplibre-gl";
  import {
    GeoJSON,
    LineLayer,
    MapEvents,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { constructMatchExpression } from "svelte-utils/map";
  import { componentColors } from "../../colors";
  import { layerId, roadLineWidth } from "../../common";
  import {
    backend,
    connectedComponents,
    map,
    mutationCounter,
    referenceRoadStyle,
  } from "../../stores";

  $: show = $referenceRoadStyle == "disconnections";

  let lastUpdate = 0;

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      $connectedComponents = await $backend.getConnectedComponents();
      lastUpdate = $mutationCounter;
    }
  }

  $: if (show && $mutationCounter > 0) {
    recalc();
  }

  let colorByComponent = constructMatchExpression(
    ["to-string", ["get", "component"]],
    Object.fromEntries(
      componentColors.map((color, i) => [i.toString(), color]),
    ),
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

<MapEvents on:click={onClick} />

<GeoJSON data={$connectedComponents} generateId>
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
