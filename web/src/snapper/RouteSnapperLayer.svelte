<script lang="ts">
  import { CircleLayer, FillLayer, GeoJSON, LineLayer } from "svelte-maplibre";
  import {
    constructMatchExpression,
    isLine,
    isPoint,
    isPolygon,
  } from "svelte-utils/map";
  import { routeToolGj, showAllNodes, showAllNodesGj } from "./stores";

  const circleRadiusPixels = 10;
</script>

<GeoJSON data={$routeToolGj}>
  <CircleLayer
    id="route-points"
    filter={isPoint}
    paint={{
      "circle-color": constructMatchExpression(
        ["get", "type"],
        {
          "snapped-waypoint": "red",
          "free-waypoint": "blue",
        },
        "black",
      ),
      "circle-opacity": ["case", ["has", "hovered"], 0.5, 1.0],
      "circle-radius": constructMatchExpression(
        ["get", "type"],
        { node: circleRadiusPixels / 2.0 },
        circleRadiusPixels,
      ),
    }}
  />
  <LineLayer
    id="route-lines"
    filter={isLine}
    paint={{
      "line-color": ["case", ["get", "snapped"], "red", "blue"],
      "line-width": 2.5,
    }}
  />
  <FillLayer
    id="route-polygons"
    filter={isPolygon}
    paint={{
      "fill-color": "black",
      "fill-opacity": 0.5,
    }}
  />
</GeoJSON>

<GeoJSON data={$showAllNodesGj}>
  <CircleLayer
    id="route-debug-nodes"
    paint={{
      "circle-opacity": 0,
      "circle-radius": 5,
      "circle-stroke-color": "black",
      "circle-stroke-width": 1,
    }}
    layout={{
      visibility: $showAllNodes ? "visible" : "none",
    }}
    minzoom={14}
  />
</GeoJSON>
