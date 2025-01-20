<script lang="ts">
  import { LineLayer, VectorTileSource } from "svelte-maplibre";
  import { constructMatchExpression } from "svelte-utils/map";
  import { levelOfServiceColors } from "../../colors";
  import { layerId, QualitativeLegend, roadLineWidth } from "../../common";
  import { assetUrl, devMode, roadStyle } from "../../stores";
  import RoadLayerControls from "../RoadLayerControls.svelte";

  $: show = $roadStyle == "los";
  let showOrig = false;
  let showCurrent = true;
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
