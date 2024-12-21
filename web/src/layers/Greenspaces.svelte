<script lang="ts">
  import {
    GeoJSON,
    hoverStateFilter,
    FillLayer,
    CircleLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { backend, type Greenspaces } from "../stores";
  import { percent } from "../utils";
  import { layerId, QualitativeLegend } from "../common";
  import { greenspaces as show } from "./stores";
  import type { Feature, MultiPolygon } from "geojson";
  import DebugReachability from "./DebugReachability.svelte";

  let data: Greenspaces = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<
    MultiPolygon,
    { reachable: boolean; idx: number }
  > | null;

  async function recalc() {
    if ($backend) {
      data = await $backend.getGreenspaces();
    }
  }

  $: if ($show && data.features.length == 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls name="Greenspaces" bind:show={$show}>
  <button class="outline" on:click={recalc}>Recalculate</button>
  <p>
    {reachable.toLocaleString()} / {data.features.length.toLocaleString()} ({percent(
      reachable,
      data.features.length,
    )}) reachable
  </p>
  <QualitativeLegend colors={{ Reachable: "purple", "Not reachable": "red" }} />
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("greenspaces")}
    manageHoverState
    paint={{
      "fill-color": ["case", ["get", "reachable"], "purple", "red"],
      "fill-opacity": hoverStateFilter(0.7, 0.9),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    bind:hovered
  >
    <Popup openOn="hover" let:props>
      Greenspace {props.name || ""}
      {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </FillLayer>

  <CircleLayer
    {...layerId("greenspace-access-points")}
    filter={["==", ["get", "kind"], "access point"]}
    paint={{
      "circle-color": "green",
      "circle-radius": 3,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>

<DebugReachability kind="greenspaces" {hovered} />
