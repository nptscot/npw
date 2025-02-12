<script lang="ts">
  import type { Feature, MultiPolygon } from "geojson";
  import {
    CircleLayer,
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { Greenspaces } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import { localPOIs as show } from "./stores";

  let lastUpdate = 0;
  let data: Greenspaces = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<
    MultiPolygon,
    { reachable: boolean; idx: number }
  > | null;

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getGreenspaces();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }
</script>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("greenspaces-fill")}
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
      Greenspace {props.name || ""}
      {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("greenspaces-outline-reachable")}
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
    {...layerId("greenspaces-outline-unreachable")}
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

  <CircleLayer
    {...layerId("greenspace-access-points")}
    interactive={false}
    filter={["==", ["get", "kind"], "access point"]}
    paint={{
      "circle-color": "green",
      "circle-radius": ["step", ["zoom"], 0, 13, 5],
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>

<DebugReachability kind="greenspaces" {hovered} />
