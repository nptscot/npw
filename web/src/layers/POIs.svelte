<script lang="ts">
  import type { ExpressionSpecification } from "maplibre-gl";
  import { GeoJSON, SymbolLayer, type LayerClickInfo } from "svelte-maplibre";
  import { layerId } from "../common";
  import { autosave, backend, mutationCounter, zoom } from "../stores";
  import type { PoiKind, POIs } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { currentPOI, localPOIs as show, type POI } from "./stores";
  import StreetViewPOI from "./StreetViewPOI.svelte";
  import WarpToPOIs from "./WarpToPOIs.svelte";

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
    currentPOI: POI | null,
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
      description: e.detail.features[0].properties!.description,
      reachable: e.detail.features[0].properties!.reachable,
      pt: e.detail.event.lngLat.toArray(),
    };
  }

  async function fixUnreachable() {
    if ($currentPOI) {
      let input = await $backend!.fixUnreachablePOI(
        $currentPOI.kind,
        $currentPOI.idx,
      );
      await $backend!.setRoute(null, input);
      await autosave();

      // TODO This assumes the fix succeeded. Can we easily check?
      $currentPOI.reachable = true;
    }
  }
</script>

<LayerControls name="POIs" bind:show={$show}>
  {#if $zoom && $zoom > 13}
    {#if $currentPOI}
      {#if $currentPOI.reachable}
        <p>
          {$currentPOI.description} is connected to the network. The blue path shows
          the route through quiet streets to the network.
        </p>
      {:else}
        <button on:click={fixUnreachable}>
          Add the black local access route to fix
        </button>
        <p>
          {$currentPOI.description} is not connected to the network. Enable the Reachability
          layer to see the red severances surronding it.
        </p>
      {/if}
    {/if}

    <WarpToPOIs />
  {:else}
    <p>Zoom in more to connect POIs</p>
  {/if}
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

<DebugReachability layerName="pois" current={$currentPOI} show={$show} />

{#if $show}
  <StreetViewPOI />
{/if}
