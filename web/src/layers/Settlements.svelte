<script lang="ts">
  import { GeoJSON, hoverStateFilter, FillLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { backend, type Settlements } from "../stores";
  import { percent } from "../utils";
  import { layerId, QualitativeLegend } from "../common";
  import { settlements as show } from "./stores";
  import type { Feature, MultiPolygon } from "geojson";
  import DebugReachability from "./DebugReachability.svelte";

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

  $: if ($show && data.features.length == 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls name="Settlements" bind:show={$show}>
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
  >
    <Popup openOn="hover" let:props>
      Settlement {props.name || ""} with population {props.population.toLocaleString()}
      {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </FillLayer>
</GeoJSON>

<DebugReachability kind="settlements" {hovered} />
