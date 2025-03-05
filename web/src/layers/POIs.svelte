<script lang="ts">
  import type { Feature, Point } from "geojson";
  import type { ExpressionSpecification } from "maplibre-gl";
  import { GeoJSON, SymbolLayer, type LayerClickInfo } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { GPHospitals, PoiKind, Schools } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { currentPOI, localPOIs as show } from "./stores";
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
  let hovered: Feature<
    Point,
    { poi_kind: PoiKind; reachable: boolean; idx: number }
  > | null;

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
    currentPOI: { kind: PoiKind; idx: number } | null,
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
    bind:hovered
    hoverCursor="pointer"
    on:click={setCurrentPOI}
  >
    <Popup openOn="hover" let:props>
      <div style="max-width: 30vw; max-height: 60vh; overflow: auto;">
        {props.name} is a {props.kind} school with {props.pupils} pupils. It {props.reachable
          ? "is"
          : "is not"} reachable. {#if !props.reachable}Click to add the black
          route to connect it to the network.{/if}
      </div>
    </Popup>
  </SymbolLayer>
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
    bind:hovered
    hoverCursor="pointer"
    on:click={setCurrentPOI}
  >
    <Popup openOn="hover" let:props>
      <div style="max-width: 30vw; max-height: 60vh; overflow: auto;">
        {props.name} is a {props.kind}. It {props.reachable ? "is" : "is not"} reachable.
        {#if !props.reachable}Click to add the black route to connect it to the
          network.{/if}
      </div>
    </Popup>
  </SymbolLayer>
</GeoJSON>

<DebugReachability layerName="pois" {hovered} />
