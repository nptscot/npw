<script lang="ts">
  import type { Feature, MultiPolygon } from "geojson";
  import { FillLayer, GeoJSON, hoverStateFilter } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, QualitativeLegend } from "../common";
  import { backend, mutationCounter, type Settlements } from "../stores";
  import { percent } from "../utils";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { settlements as show } from "./stores";

  let data: Settlements = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<
    MultiPolygon,
    { reachable: boolean; idx: number }
  > | null;

  async function recalc() {
    if ($backend) {
      data = await $backend.getSettlements();
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
  <QualitativeLegend colors={{ Reachable: "purple", "Not reachable": "red" }} />
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("settlements")}
    manageHoverState
    paint={{
      "fill-color": ["case", ["get", "reachable"], "purple", "red"],
      "fill-opacity": hoverStateFilter(0.7, 0.9),
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
</GeoJSON>

<DebugReachability kind="settlements" {hovered} />
