<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { GeoJSON, LineLayer, VectorTileSource } from "svelte-maplibre";
  import { constructMatchExpression, Popup } from "svelte-utils/map";
  import { colorByLoS, levelOfServiceColors } from "../colors";
  import { layerId, QualitativeLegend, roadLineWidth } from "../common";
  import {
    assetUrl,
    backend,
    devMode,
    mutationCounter,
    roadStyle,
  } from "../stores";
  import RoadLayerControls from "./RoadLayerControls.svelte";

  let lastUpdate = 0;
  let data: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  $: show = $roadStyle == "los";
  let showOrig = false;
  let showCurrent = true;

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.renderLevelOfService();
      lastUpdate = $mutationCounter;
    }
  }

  $: if (show && showCurrent && $mutationCounter > 0) {
    recalc();
  }
</script>

<RoadLayerControls name="Level of Service" style="los">
  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={showOrig} />
      Show original data
    </label>

    <label>
      <input type="checkbox" bind:checked={showCurrent} />
      Show current derived data
    </label>
  {/if}

  <QualitativeLegend colors={levelOfServiceColors} />
</RoadLayerControls>

<!-- TODO Continue showing this for debugging the map matching -->
<VectorTileSource url={`pmtiles://${assetUrl("cbd.pmtiles")}`}>
  <LineLayer
    {...layerId("los-debug")}
    sourceLayer="cbd_layer"
    paint={{
      "line-color": constructMatchExpression(
        ["get", "Level of Service"],
        {
          High: "mediumseagreen",
          Medium: "orange",
          Low: "red",
          "Should not be used": "brown",
        },
        "cyan",
      ),
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility: show && showOrig ? "visible" : "none",
    }}
  />
</VectorTileSource>

<GeoJSON {data} generateId>
  <LineLayer
    {...layerId("los")}
    layout={{
      visibility: show && showCurrent ? "visible" : "none",
    }}
    paint={{
      "line-width": roadLineWidth(0),
      "line-color": colorByLoS,
      "line-opacity": 0.8,
    }}
    manageHoverState
  >
    <Popup openOn="hover" let:props>{props.los}</Popup>
  </LineLayer>
</GeoJSON>
