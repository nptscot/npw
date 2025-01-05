<script lang="ts">
  import { GeoJSON, LineLayer, VectorTileSource } from "svelte-maplibre";
  import { SequentialLegend } from "svelte-utils";
  import { makeRamp, Popup } from "svelte-utils/map";
  import { layerId, roadLineWidth } from "../common";
  import { assetUrl, backend } from "../stores";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let showOrig = false;
  let showMatched = true;
  let firstLoad = false;

  $: if (show && showMatched) {
    firstLoad = true;
  }

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

<LayerControls name="Estimated speed limit" bind:show>
  <label>
    <input type="checkbox" bind:checked={showOrig} />
    Show original data
  </label>

  <label>
    <input type="checkbox" bind:checked={showMatched} />
    Show map-matched data
  </label>

  <SequentialLegend {colorScale} {limits} />
</LayerControls>

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

{#if $backend && firstLoad}
  {#await $backend.renderLevelOfService() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("speed_limit")}
        layout={{
          visibility: show && showMatched ? "visible" : "none",
        }}
        paint={{
          "line-width": roadLineWidth(0),
          "line-color": makeRamp(["get", "speed"], limits, colorScale),
          "line-opacity": 0.8,
        }}
        manageHoverState
      >
        <Popup openOn="hover" let:props>
          {props.speed}
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
