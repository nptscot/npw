<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { backend, infraTypeMapping } from "../stores";
  import { colorByInraType } from "../common";
  import { Popup } from "svelte-utils/map";
  import { notNull } from "svelte-utils";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Existing network
  </label>
</LayerControls>

{#if $backend}
  {#await $backend.classifyExistingNetwork() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        layout={{
          visibility: show ? "visible" : "none",
        }}
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": colorByInraType,
          "line-opacity": 0.8,
        }}
        manageHoverState
        on:click={(e) =>
          window.open(notNull(e.detail.features[0].properties).way, "_blank")}
        hoverCursor="pointer"
      >
        <Popup openOn="hover" let:props>
          {infraTypeMapping[props.infra_type][0]}
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
