<script lang="ts">
  import type { Feature, Point } from "geojson";
  import { CircleLayer, GeoJSON, hoverStateFilter } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, QualitativeLegend } from "../common";
  import { backend, mutationCounter, type GPHospitals } from "../stores";
  import { percent } from "../utils";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { gpHospitals as show } from "./stores";

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

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls name="GPs and hospitals" bind:show={$show}>
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
    hoverCursor="pointer"
  >
    <Popup openOn="click" let:props>
      {props.name} is a {props.kind}. It {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </CircleLayer>
</GeoJSON>

<DebugReachability kind="gp_hospitals" {hovered} />
