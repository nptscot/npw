<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import type { ExpressionSpecification } from "maplibre-gl";
  import {
    CircleLayer,
    GeoJSON,
    SymbolLayer,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { emptyGeojson } from "svelte-utils/map";
  import { layerId } from "../common";
  import { localPOIs as show } from "../layers/stores";
  import { backend, mutationCounter } from "../stores";
  import type { PoiKind, POIs } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import { currentPOI, type POI } from "./stores";

  export let filterKind: PoiKind | "all" = "all";

  let lastUpdate = 0;
  let schools: POIs = {
    type: "FeatureCollection",
    features: [],
  };
  let gpHospitals: POIs = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      let gj = await $backend.getPOIs();
      // It's easiest to filter upfront here, to simplify iconImage
      schools.features = gj.features.filter(
        (f) => f.properties.poi_kind == "schools",
      );
      schools = schools;
      gpHospitals.features = gj.features.filter(
        (f) => f.properties.poi_kind == "gp_hospitals",
      );
      gpHospitals = gpHospitals;
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }

  function iconImage(
    poiKind: PoiKind,
    filterKind: PoiKind | "all",
  ): ExpressionSpecification {
    if (filterKind != "all" && filterKind != poiKind) {
      // @ts-expect-error
      return `${poiKind}_ignore`;
    }

    return [
      "case",
      ["get", "reachable"],
      `${poiKind}_reachable`,
      `${poiKind}_unreachable`,
    ] as ExpressionSpecification;
  }

  function setCurrentPOI(e: CustomEvent<LayerClickInfo>) {
    // Find the original feature, to set the point correctly
    let collection =
      e.detail.features[0].properties!.poi_kind == "schools"
        ? schools
        : gpHospitals;
    let feature = collection.features.find(
      (f) => f.properties.idx == e.detail.features[0].properties!.idx,
    );
    if (feature) {
      $currentPOI = {
        kind: feature.properties.poi_kind,
        idx: feature.properties.idx,
        description: feature.properties.description,
        reachable: feature.properties.reachable,
        pt: feature.geometry.coordinates as [number, number],
      };
    }
  }

  function focusCurrentPOI(currentPOI: POI | null): FeatureCollection {
    let gj = emptyGeojson();
    if (currentPOI && currentPOI.kind != "greenspaces") {
      gj.features.push({
        type: "Feature",
        properties: {},
        geometry: {
          type: "Point",
          coordinates: currentPOI.pt,
        },
      });
    }
    return gj;
  }
</script>

<GeoJSON data={schools} generateId>
  <SymbolLayer
    {...layerId("schools")}
    manageHoverState
    layout={{
      visibility: $show ? "visible" : "none",
      "icon-allow-overlap": true,
      "icon-size": ["interpolate", ["linear"], ["zoom"], 10, 0.1, 12, 1.0],
      "icon-image": iconImage("schools", filterKind),
    }}
    hoverCursor="pointer"
    on:click={setCurrentPOI}
  />
</GeoJSON>

<GeoJSON data={gpHospitals} generateId>
  <SymbolLayer
    {...layerId("gp-hospitals")}
    manageHoverState
    layout={{
      visibility: $show ? "visible" : "none",
      "icon-allow-overlap": true,
      "icon-size": ["interpolate", ["linear"], ["zoom"], 10, 0.1, 12, 1.0],
      "icon-image": iconImage("gp_hospitals", filterKind),
    }}
    hoverCursor="pointer"
    on:click={setCurrentPOI}
  />
</GeoJSON>

<GeoJSON data={focusCurrentPOI($currentPOI)}>
  <CircleLayer
    {...layerId("current-poi")}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    paint={{
      "circle-radius": 15,
      "circle-opacity": 0,
      "circle-stroke-color": "black",
      "circle-stroke-width": 4,
    }}
  />
</GeoJSON>

<DebugReachability current={$currentPOI} show={$show} />
