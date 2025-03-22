<script lang="ts">
  import type { ExpressionSpecification } from "maplibre-gl";
  import {
    CircleLayer,
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { Greenspaces } from "../types";
  import { currentPOI, localPOIs as show, type POI } from "./stores";

  let lastUpdate = 0;
  let data: Greenspaces = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getGreenspaces();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }

  function fillOpacity(currentPOI: POI | null): ExpressionSpecification {
    if (currentPOI?.kind == "greenspaces") {
      return [
        "case",
        ["==", ["get", "idx"], currentPOI.idx],
        0.8,
        hoverStateFilter(0.1, 0.5),
      ];
    } else {
      return hoverStateFilter(0.1, 0.5);
    }
  }

  function setCurrentPOI(e: CustomEvent<LayerClickInfo>) {
    $currentPOI = {
      kind: e.detail.features[0].properties!.poi_kind,
      idx: e.detail.features[0].properties!.idx,
      description: e.detail.features[0].properties!.description,
      reachable: e.detail.features[0].properties!.reachable,
      pt: e.detail.event.lngLat.toArray(),
    };
  }
</script>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("greenspaces-fill")}
    manageHoverState
    paint={{
      "fill-color": ["case", ["get", "reachable"], "green", "red"],
      "fill-opacity": fillOpacity($currentPOI),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    hoverCursor="pointer"
    on:click={setCurrentPOI}
  />

  <LineLayer
    {...layerId("greenspaces-outline-reachable")}
    interactive={false}
    filter={["get", "reachable"]}
    paint={{
      "line-color": "green",
      "line-width": 1,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
  <LineLayer
    {...layerId("greenspaces-outline-unreachable")}
    interactive={false}
    filter={["!", ["get", "reachable"]]}
    paint={{
      "line-color": "red",
      "line-width": 1,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />

  <CircleLayer
    {...layerId("greenspace-access-points")}
    interactive={false}
    filter={["==", ["get", "kind"], "access point"]}
    paint={{
      "circle-color": "grey",
      "circle-radius": ["step", ["zoom"], 0, 14, 5],
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
