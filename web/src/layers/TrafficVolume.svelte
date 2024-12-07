<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { backend } from "../stores";
  import { Popup, makeRamp } from "svelte-utils/map";
  import { SequentialLegend } from "svelte-utils";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }

  let colorScale = ["#27918d", "#ffaa33", "#440154"];
  let limits = [0, 2000, 4000, 10000];
</script>

<LayerControls name="traffic volume">
  <label>
    <input type="checkbox" bind:checked={show} />
    Estimated traffic volume
  </label>

  {#if show}
    <SequentialLegend {colorScale} {limits} />
  {/if}
</LayerControls>

{#if $backend && firstLoad}
  {#await $backend.renderLevelOfService() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        layout={{
          visibility: show ? "visible" : "none",
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
