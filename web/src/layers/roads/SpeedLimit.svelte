<script lang="ts">
  import { LineLayer, VectorTileSource } from "svelte-maplibre";
  import { makeRamp } from "svelte-utils/map";
  import { layerId, roadLineWidth } from "../../common";
  import { assetUrl, devMode, roadStyle } from "../../stores";
  import RoadLayerControls from "../RoadLayerControls.svelte";

  $: show = $roadStyle == "speed";
  let showOrig = false;
  let showMatched = true;

  let colorScale = [
    "#8a9a5b",
    "#ffc300",
    "#cc5500",
    "#c70039",
    "#900c3f",
    "#581845",
  ];
  let limits = [20, 30, 40, 50, 60, 70];
</script>

<RoadLayerControls name="Estimated speed limit" style="speed">
  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={showOrig} />
      Show original data
    </label>

    <label>
      <input type="checkbox" bind:checked={showMatched} />
      Show map-matched data
    </label>
  {/if}
</RoadLayerControls>

<!-- TODO Continue showing this for debugging the map matching -->
<VectorTileSource url={`pmtiles://${assetUrl("cbd.pmtiles")}`}>
  <LineLayer
    {...layerId("speed_limit-debug")}
    sourceLayer="cbd_layer"
    paint={{
      "line-color": makeRamp(["get", "Speed limit"], limits, colorScale),
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility: show && showOrig ? "visible" : "none",
    }}
  />
</VectorTileSource>
