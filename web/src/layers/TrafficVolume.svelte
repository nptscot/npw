<script lang="ts">
  import { GeoJSON, LineLayer, VectorTileSource } from "svelte-maplibre";
  import { SequentialLegend } from "svelte-utils";
  import { constructMatchExpression, makeRamp, Popup } from "svelte-utils/map";
  import { layerId, roadLineWidth } from "../common";
  import { assetUrl, backend, devMode, roadStyle } from "../stores";
  import RoadLayerControls from "./RoadLayerControls.svelte";

  $: show = $roadStyle == "traffic";
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }

  let colorScale = ["#27918d", "#ffaa33", "#440154"];
  let limits = [0, 2000, 4000, 10000];

  let showTruth = false;
  let showMatched = true;
</script>

<RoadLayerControls name="Estimated traffic volume" style="traffic">
  <SequentialLegend {colorScale} {limits} />

  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={showTruth} />
      Show actual data
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
    {...layerId("traffic-debug")}
    sourceLayer="cbd_layer"
    filter={["has", "Traffic volume category"]}
    paint={{
      "line-color": constructMatchExpression(
        ["get", "Traffic volume category"],
        {
          "0 to 1999": "#27918d",
          "2000 to 3999": "#ffaa33",
          "4000+": "#440154",
        },
        "cyan",
      ),
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility: show && showTruth ? "visible" : "none",
    }}
  />
</VectorTileSource>

{#if $backend && firstLoad}
  {#await $backend.renderStaticRoads() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("traffic")}
        layout={{
          visibility: show && showMatched ? "visible" : "none",
        }}
        paint={{
          "line-width": roadLineWidth(0),
          "line-color": makeRamp(["get", "traffic"], limits, colorScale),
          "line-opacity": 0.8,
        }}
        manageHoverState
      >
        <Popup openOn="hover" let:props>
          <p>{props.traffic.toLocaleString()}</p>
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
