<script lang="ts">
  import type { Feature, MultiPolygon } from "geojson";
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { QualitativeLegend } from "svelte-utils";
  import { Popup } from "svelte-utils/map";
  import { layerId, percent } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { TownCentres } from "../types";
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
    { reachable: boolean; idx: number }
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

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls name="Town centres" bind:show={$show}>
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
    {...layerId("town-centres-fill")}
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
      Town centre {props.name || ""}
      {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("town-centres-outline")}
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

<DebugReachability kind="town_centres" {hovered} />
