<script lang="ts">
  import { GeoJSON, LineLayer, VectorTileSource } from "svelte-maplibre";
  import { Loading } from "svelte-utils";
  import { constructMatchExpression } from "svelte-utils/map";
  import { colorByTier } from "../colors";
  import { layerId, roadLineWidth } from "../common";
  import { assetUrl, autosave, backend } from "../stores";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let firstLoad = false;
  let loading = "";

  $: if (show) {
    firstLoad = true;
  }

  let showTruth = false;
  let showMatched = true;

  async function importExisting() {
    if ($backend) {
      loading = "Importing core network";
      let numChanges = await $backend.importCoreNetwork();
      loading = "";
      let noun = numChanges == 1 ? "route segment" : "route segments";
      await autosave();
      window.alert(`Imported ${numChanges} ${noun}`);
    }
  }
</script>

<Loading {loading} />

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
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility: show && showTruth ? "visible" : "none",
    }}
  />
</VectorTileSource>

{#if $backend && firstLoad}
  {#await $backend.renderCoreNetwork() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("cn")}
        paint={{
          "line-color": colorByTier,
          "line-width": roadLineWidth(0),
        }}
        layout={{
          visibility: show && showMatched ? "visible" : "none",
        }}
      />
    </GeoJSON>
  {/await}
{/if}
