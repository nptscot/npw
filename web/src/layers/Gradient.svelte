<script lang="ts">
  import { layerId } from "../common";
  import { GeoJSON, LineLayer, hoverStateFilter } from "svelte-maplibre";
  import { backend } from "../stores";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { lineColorForGradient } from "../utils";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }

  // TODO Legend
</script>

<LayerControls name="gradient">
  <label>
    <input type="checkbox" bind:checked={show} />
    Gradient
  </label>
</LayerControls>

{#if $backend && firstLoad}
  {#await $backend.renderLevelOfService() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("gradients")}
        layout={{
          visibility: show ? "visible" : "none",
        }}
        paint={{
          "line-width": hoverStateFilter(3, 5),
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
