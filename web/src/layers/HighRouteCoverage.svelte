<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { backend } from "../stores";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { highRouteCoverage as show } from "./stores";
  import { lineWidthForDemand, lineColorForDemand } from "../utils";

  let firstLoad = false;

  $: if ($show) {
    firstLoad = true;
  }
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={$show} />
    High NPT route coverage
  </label>

  {#if $show}
    legend
  {/if}
</LayerControls>

{#if $backend && firstLoad}
  {#await $backend.renderPrecalculatedFlows() then data}
    <GeoJSON {data}>
      <LineLayer
        layout={{
          visibility: $show ? "visible" : "none",
        }}
        paint={{
          "line-width": lineWidthForDemand("flow"),
          "line-color": lineColorForDemand("flow"),
        }}
      >
        <Popup openOn="hover" let:props>
          Flow {props.flow.toLocaleString()}
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
