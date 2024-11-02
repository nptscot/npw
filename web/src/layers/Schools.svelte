<script lang="ts">
  import { GeoJSON, hoverStateFilter, CircleLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { backend } from "../stores";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Schools
  </label>
</LayerControls>

{#if $backend && firstLoad}
  {#await $backend.getSchools() then data}
    <GeoJSON {data} generateId>
      <CircleLayer
        manageHoverState
        paint={{
          "circle-color": "black",
          "circle-radius": hoverStateFilter(5, 8),
        }}
        layout={{
          visibility: show ? "visible" : "none",
        }}
      >
        <Popup openOn="hover" let:props>
          {props.name} is a {props.kind} school with {props.pupils} pupils
        </Popup>
      </CircleLayer>
    </GeoJSON>
  {/await}
{/if}
