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
  import { localPOIs as show } from "../layers/stores";
  import { backend, mutationCounter } from "../stores";
  import type { Greenspaces, PoiKind } from "../types";
  import { currentPOI, type POI } from "./stores";

  export let filterKind: PoiKind | "all" = "all";

  $: showGreenspaces = filterKind == "all" || filterKind == "greenspaces";

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
      "fill-color": showGreenspaces
        ? ["case", ["get", "reachable"], "green", "red"]
        : "#666666",
      "fill-opacity": fillOpacity($currentPOI),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    hoverCursor="pointer"
    on:click={setCurrentPOI}
  />

  <LineLayer
    {...layerId("greenspaces-outline")}
    interactive={false}
    paint={{
      "line-color": showGreenspaces
        ? ["case", ["get", "reachable"], "green", "red"]
        : "#666666",
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
      visibility: $show && showGreenspaces ? "visible" : "none",
    }}
  />
</GeoJSON>
