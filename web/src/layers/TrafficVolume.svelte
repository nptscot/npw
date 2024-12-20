<script lang="ts">
  import { GeoJSON, LineLayer, VectorTileSource } from "svelte-maplibre";
  import { backend, assetUrl } from "../stores";
  import { Popup, makeRamp, constructMatchExpression } from "svelte-utils/map";
  import { SequentialLegend } from "svelte-utils";
  import { layerId } from "../common";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }

  let colorScale = ["#27918d", "#ffaa33", "#440154"];
  let limits = [0, 2000, 4000, 10000];

  let showTruth = true;
  let showMatched = true;
</script>

<LayerControls name="Estimated traffic volume" bind:show>
  <SequentialLegend {colorScale} {limits} />

  <label>
    <input type="checkbox" bind:checked={showTruth} />
    Show actual data
  </label>

  <label>
    <input type="checkbox" bind:checked={showMatched} />
    Show map-matched data
  </label>
</LayerControls>

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
      "line-width": 5,
    }}
    layout={{
      visibility: show && showTruth ? "visible" : "none",
    }}
  />
</VectorTileSource>

{#if $backend && firstLoad}
  {#await $backend.renderLevelOfService() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("traffic")}
        layout={{
          visibility: show && showMatched ? "visible" : "none",
        }}
        paint={{
          "line-width": makeRamp(["get", "traffic"], limits, [1, 2, 3]),
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
