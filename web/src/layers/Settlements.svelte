<script lang="ts">
  import type { Feature, MultiPolygon } from "geojson";
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, percent, QualitativeLegend } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { Settlements } from "../types";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { settlements as show } from "./stores";

  let lastUpdate = 0;
  let data: Settlements = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<
    MultiPolygon,
    { reachable: boolean; idx: number }
  > | null;

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getSettlements();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls name="Settlements" bind:show={$show}>
  <p>
    {reachable.toLocaleString()} / {data.features.length.toLocaleString()} ({percent(
      reachable,
      data.features.length,
    )}) reachable
  </p>
  <QualitativeLegend
    horiz
    colors={{ Reachable: "purple", "Not reachable": "red" }}
  />
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("settlements-fill")}
    manageHoverState
    paint={{
      "fill-color": ["case", ["get", "reachable"], "purple", "red"],
      "fill-opacity": hoverStateFilter(0.1, 0.9),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    bind:hovered
    hoverCursor="pointer"
  >
    <Popup openOn="click" let:props>
      Settlement {props.name || ""} with population {props.population.toLocaleString()}
      {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("settlements-outline")}
    interactive={false}
    paint={{
      "line-color": ["case", ["get", "reachable"], "purple", "red"],
      "line-width": 2,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>

<DebugReachability kind="settlements" {hovered} />
