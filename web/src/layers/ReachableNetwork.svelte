<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { constructMatchExpression } from "svelte-utils/map";
  import { layerId, QualitativeLegend, roadLineWidth } from "../common";
  import { backend, mutationCounter } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import { severances } from "./stores";

  let show = false;

  let lastUpdate = 0;
  let data: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.renderReachableNetwork();
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

<LayerControls name="Reachable network" bind:show>
  <QualitativeLegend {colors} />
</LayerControls>

<GeoJSON {data}>
  <LineLayer
    {...layerId("reachability")}
    layout={{
      visibility: show || $severances ? "visible" : "none",
    }}
    filter={$severances ? ["==", ["get", "kind"], "severance"] : undefined}
    paint={{
      "line-width": roadLineWidth(0),
      "line-color": constructMatchExpression(["get", "kind"], colors, "black"),
      "line-opacity": 0.8,
    }}
  />
</GeoJSON>
