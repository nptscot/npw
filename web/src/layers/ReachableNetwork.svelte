<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { backend } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import type { FeatureCollection } from "geojson";
  import { constructMatchExpression } from "svelte-utils/map";
  import { layerId, QualitativeLegend } from "../common";

  // TODO Does this belong as a layer like this, or a debug mode, in the short term?

  let show = false;

  let data: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    data = await $backend!.renderReachableNetwork();
  }

  $: if (show && data.features.length == 0) {
    recalc();
  }

  let colors = {
    network: "green",
    reachable: "purple",
    severance: "red",
  };
</script>

<LayerControls name="Reachable network" bind:show>
  <button class="outline" on:click={recalc}>Recalculate</button>
  <QualitativeLegend {colors} />
</LayerControls>

<GeoJSON {data} generateId>
  <LineLayer
    {...layerId("reachability")}
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "line-width": hoverStateFilter(5, 7),
      "line-color": constructMatchExpression(["get", "kind"], colors, "black"),
      "line-opacity": 0.8,
    }}
    manageHoverState
  />
</GeoJSON>
