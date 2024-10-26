<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { assetUrl } from "../stores";

  let show = false;
  let purpose = "all";
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Route network
  </label>

  {#if show}
    <label>
      Trip purpose:
      <select bind:value={purpose}>
        {#each [["all", "All"], ["commute", "Commute"], ["primary", "Primary School"], ["secondary", "Secondary"], ["utility", "Other everyday"]] as [value, label]}
          <option {value}>{label}</option>
        {/each}
      </select>
    </label>
  {/if}
</LayerControls>

<VectorTileSource url={`pmtiles://${assetUrl("route_network.pmtiles")}`}>
  <LineLayer
    sourceLayer="rnet"
    manageHoverState
    paint={{
      "line-color": "black",
      "line-width": hoverStateFilter(2, 3),
    }}
    layout={{
      visibility: show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props[`${purpose}_fastest_bicycle`]}
    </Popup>
  </LineLayer>
</VectorTileSource>
