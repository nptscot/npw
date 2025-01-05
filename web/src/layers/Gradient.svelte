<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, roadLineWidth } from "../common";
  import { backend } from "../stores";
  import { lineColorForGradient } from "../utils";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }

  // TODO Legend
</script>

<LayerControls name="Gradient" bind:show />

{#if $backend && firstLoad}
  {#await $backend.renderLevelOfService() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("gradients")}
        layout={{
          visibility: show ? "visible" : "none",
        }}
        paint={{
          "line-width": roadLineWidth(0),
          "line-color": lineColorForGradient(),
        }}
        manageHoverState
      >
        <Popup openOn="hover" let:props>
          <p>{props.gradient.toFixed(1)}%</p>
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
