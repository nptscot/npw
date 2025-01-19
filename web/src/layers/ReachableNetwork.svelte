<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { constructMatchExpression } from "svelte-utils/map";
  import { layerId, QualitativeLegend, roadLineWidth } from "../common";
  import { backend, mutationCounter, roadStyle } from "../stores";
  import RoadLayerControls from "./RoadLayerControls.svelte";
  import { severances } from "./stores";

  $: show = $roadStyle == "reachability";

  let lastUpdate = 0;
  let data: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.renderDynamicRoads();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ((show || $severances) && $mutationCounter > 0) {
    recalc();
  }

  let colors = {
    network: "green",
    reachable: "purple",
    severance: "red",
  };
</script>

<RoadLayerControls name="Reachable network" style="reachability">
  <QualitativeLegend {colors} />
</RoadLayerControls>

<GeoJSON {data}>
  <LineLayer
    {...layerId("reachability")}
    layout={{
      visibility: show || $severances ? "visible" : "none",
    }}
    filter={$severances
      ? ["==", ["get", "reachable"], "severance"]
      : ["!=", ["get", "reachable"], "unreachable"]}
    paint={{
      "line-width": roadLineWidth(0),
      "line-color": constructMatchExpression(
        ["get", "reachable"],
        colors,
        "black",
      ),
      "line-opacity": 0.8,
    }}
  />
</GeoJSON>
