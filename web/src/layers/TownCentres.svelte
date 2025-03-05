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
  import { backend, mutationCounter } from "../stores";
  import type { PoiKind, TownCentres } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
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

<LayerControls name="Town centres" bind:show={$show} empty />

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("town-centres-fill")}
    manageHoverState
    paint={{
      "fill-color": ["case", ["get", "reachable"], "green", "red"],
      "fill-opacity": hoverStateFilter(0.0, 0.5),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    bind:hovered
    hoverCursor="pointer"
  >
    <Popup openOn="click" let:props>
      Town centre {props.name || ""}
      {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("town-centres-outline-reachable")}
    interactive={false}
    filter={["get", "reachable"]}
    paint={{
      "line-color": "green",
      "line-width": 2,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
  <LineLayer
    {...layerId("town-centres-outline-unreachable")}
    interactive={false}
    filter={["!", ["get", "reachable"]]}
    paint={{
      "line-color": "red",
      "line-width": 2,
      "line-dasharray": [3, 2],
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>

<DebugReachability layerName="town_centres" {hovered} />
