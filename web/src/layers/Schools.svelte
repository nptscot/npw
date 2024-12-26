<script lang="ts">
  import type { Feature, Point } from "geojson";
  import { GeoJSON, SymbolLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, QualitativeLegend } from "../common";
  import { backend, mutationCounter, type Schools } from "../stores";
  import { percent } from "../utils";
  import DebugReachability from "./DebugReachability.svelte";
  import LayerControls from "./LayerControls.svelte";
  import { schools as show } from "./stores";

  let data: Schools = {
    type: "FeatureCollection",
    features: [],
  };
  let hovered: Feature<Point, { reachable: boolean; idx: number }> | null;

  async function recalc() {
    if ($backend) {
      data = await $backend.getSchools();
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls name="Schools" bind:show={$show}>
  <p>
    {reachable.toLocaleString()} / {data.features.length.toLocaleString()} ({percent(
      reachable,
      data.features.length,
    )}) reachable
  </p>
  <QualitativeLegend colors={{ Reachable: "purple", "Not reachable": "red" }} />
</LayerControls>

<GeoJSON {data} generateId>
  <SymbolLayer
    {...layerId("schools")}
    manageHoverState
    layout={{
      visibility: $show ? "visible" : "none",
      "icon-allow-overlap": true,
      "icon-size": 0.5,
      "icon-image": [
        "case",
        ["get", "reachable"],
        "school_reachable",
        "school_unreachable",
      ],
    }}
    bind:hovered
    hoverCursor="pointer"
  >
    <Popup openOn="click" let:props>
      {props.name} is a {props.kind} school with {props.pupils} pupils. It {props.reachable
        ? "is"
        : "is not"} reachable.
    </Popup>
  </SymbolLayer>
</GeoJSON>

<DebugReachability kind="schools" {hovered} />
