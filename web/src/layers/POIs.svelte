<script lang="ts">
  import type { ExpressionSpecification } from "maplibre-gl";
  import { GeoJSON, SymbolLayer, type LayerClickInfo } from "svelte-maplibre";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { GPHospitals, PoiKind, Schools } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { currentPOI, localPOIs as show, type CurrentPOI } from "./stores";
  import WarpToPOIs from "./WarpToPOIs.svelte";

  let lastUpdate = 0;
  let schools: Schools = {
    type: "FeatureCollection",
    features: [],
  };
  let gpHospitals: GPHospitals = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      schools = await $backend.getSchools();
      gpHospitals = await $backend.getGpHospitals();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }

  function iconImage(
    poiKind: PoiKind,
    currentPOI: CurrentPOI | null,
  ): ExpressionSpecification {
    let reachable = [
      "case",
      ["get", "reachable"],
      `${poiKind}_reachable`,
      `${poiKind}_unreachable`,
    ] as ExpressionSpecification;
    if (currentPOI?.kind == poiKind) {
      return [
        "case",
        ["==", ["get", "idx"], currentPOI.idx],
        "current_poi",
        reachable,
      ];
    } else {
      return reachable;
    }
  }

  function setCurrentPOI(e: CustomEvent<LayerClickInfo>) {
    $currentPOI = {
      kind: e.detail.features[0].properties!.poi_kind,
      idx: e.detail.features[0].properties!.idx,
      reachable: e.detail.features[0].properties!.reachable,
    };
  }
</script>

<LayerControls name="POIs" bind:show={$show}>
  <WarpToPOIs />
</LayerControls>

<GeoJSON data={schools} generateId>
  <SymbolLayer
    {...layerId("schools")}
    manageHoverState
    layout={{
      visibility: $show ? "visible" : "none",
      "icon-allow-overlap": true,
      "icon-size": ["interpolate", ["linear"], ["zoom"], 10, 0.1, 12, 1.0],
      "icon-image": iconImage("schools", $currentPOI),
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
      "icon-image": iconImage("gp_hospitals", $currentPOI),
    }}
    hoverCursor="pointer"
    on:click={setCurrentPOI}
  />
</GeoJSON>

<DebugReachability layerName="pois" current={$currentPOI} />
