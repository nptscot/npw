<script lang="ts">
  import type { Feature, MultiPolygon } from "geojson";
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import DebugReachability from "../local_access/DebugReachability.svelte";
  import { backend, mutationCounter } from "../stores";
  import type { PoiKind, TownCentres } from "../types";
  import { townCentres as show } from "./stores";

  let lastUpdate = 0;
  let data: TownCentres = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<
    MultiPolygon,
    { poi_kind: PoiKind; reachable: boolean; idx: number }
  > | null;

  $: hoveredPOI = hovered
    ? {
        kind: hovered.properties.poi_kind,
        idx: hovered.properties.idx,
        reachable: hovered.properties.reachable,
      }
    : null;

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getTownCentres();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }
</script>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("town-centres-fill")}
    manageHoverState
    paint={{
      "fill-color": ["case", ["get", "reachable"], "green", "red"],
      "fill-opacity": hoverStateFilter(0.1, 0.5),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    bind:hovered
    hoverCursor="pointer"
  >
    <Popup openOn="click" let:props>
      {props.description}
      {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("town-centres-outline")}
    interactive={false}
    paint={{
      "line-color": ["case", ["get", "reachable"], "green", "red"],
      "line-width": 1,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>

<DebugReachability layerName="town_centres" current={hoveredPOI} show={$show} />
