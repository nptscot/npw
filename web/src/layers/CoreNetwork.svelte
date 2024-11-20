<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    LineLayer,
    GeoJSON,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { assetUrl, backend } from "../stores";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Core network
  </label>
</LayerControls>

<!-- TODO Continue showing this for debugging the map matching -->
<VectorTileSource url={`pmtiles://${assetUrl("core_network.pmtiles")}`}>
  <LineLayer
    sourceLayer="coherent_networks"
    manageHoverState
    paint={{
      "line-color": "black",
      "line-width": 2,
    }}
    layout={{
      visibility: show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      Go Dutch value {props.all_fastest_bicycle_go_dutch}
    </Popup>
  </LineLayer>
</VectorTileSource>

{#if $backend && firstLoad}
  {#await $backend.renderCoreNetwork() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        manageHoverState
        paint={{
          "line-color": "red",
          "line-width": hoverStateFilter(2, 3),
        }}
        layout={{
          visibility: show ? "visible" : "none",
        }}
      />
    </GeoJSON>
  {/await}
{/if}
