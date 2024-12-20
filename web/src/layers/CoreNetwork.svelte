<script lang="ts">
  import { layerId } from "../common";
  import { VectorTileSource, LineLayer, GeoJSON } from "svelte-maplibre";
  import { Popup, constructMatchExpression } from "svelte-utils/map";
  import {
    assetUrl,
    backend,
    mainModeRoutesChanged,
    autosave,
  } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import { colorByTier } from "../colors";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }

  let showTruth = true;
  let showMatched = true;

  async function importExisting() {
    if ($backend) {
      let numChanges = $backend.importCoreNetwork();
      let noun = numChanges == 1 ? "route segment" : "route segments";
      await autosave();
      $mainModeRoutesChanged += 1;
      window.alert(`Imported ${numChanges} ${noun}`);
    }
  }
</script>

<LayerControls name="Core network" bind:show>
  <button class="outline" on:click={importExisting}>Import core network</button>

  <label>
    <input type="checkbox" bind:checked={showTruth} />
    Show actual core network
  </label>

  <label>
    <input type="checkbox" bind:checked={showMatched} />
    Show map-matched core network
  </label>
</LayerControls>

<!-- TODO Continue showing this for debugging the map matching -->
<VectorTileSource url={`pmtiles://${assetUrl("core_network.pmtiles")}`}>
  <LineLayer
    {...layerId("cn-debug")}
    sourceLayer="coherent_networks"
    paint={{
      "line-color": constructMatchExpression(
        ["get", "road_function"],
        {
          Primary: "#c00000",
          Secondary: "#e97132",
          "Local Access": "#ffc000",
          "Long Distance": "#4ea72e",
        },
        "cyan",
      ),
      "line-width": 2,
    }}
    layout={{
      visibility: show && showTruth ? "visible" : "none",
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
        {...layerId("cn")}
        paint={{
          "line-color": colorByTier,
          "line-width": 10,
          "line-opacity": 0.5,
        }}
        layout={{
          visibility: show && showMatched ? "visible" : "none",
        }}
      />
    </GeoJSON>
  {/await}
{/if}
