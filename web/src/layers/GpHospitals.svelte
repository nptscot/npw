<script lang="ts">
  import { GeoJSON, hoverStateFilter, CircleLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { backend, type GPHospitals } from "../stores";
  import { percent } from "../utils";
  import { layerId, QualitativeLegend } from "../common";
  import { gpHospitals as show } from "./stores";
  import type { Feature, Point } from "geojson";
  import DebugReachability from "./DebugReachability.svelte";

  let data: GPHospitals = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<Point, { reachable: boolean; idx: number }> | null;

  async function recalc() {
    if ($backend) {
      data = await $backend.getGpHospitals();
    }
  }

  $: if ($show && data.features.length == 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls name="GPs and hospitals" bind:show={$show}>
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
  <CircleLayer
    {...layerId("gp-hospitals")}
    manageHoverState
    paint={{
      "circle-color": ["case", ["get", "reachable"], "purple", "red"],
      "circle-radius": hoverStateFilter(5, 8),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    bind:hovered
  >
    <Popup openOn="hover" let:props>
      {props.name} is a {props.kind}. It {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </CircleLayer>
</GeoJSON>

<DebugReachability kind="gp_hospitals" {hovered} />
