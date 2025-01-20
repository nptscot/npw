<script lang="ts">
  import { LineLayer, VectorTileSource } from "svelte-maplibre";
  import { Loading } from "svelte-utils";
  import { constructMatchExpression } from "svelte-utils/map";
  import { layerId, roadLineWidth } from "../../common";
  import {
    assetUrl,
    autosave,
    backend,
    devMode,
    roadStyle,
  } from "../../stores";
  import RoadLayerControls from "../RoadLayerControls.svelte";

  let loading = "";

  $: show = $roadStyle == "cn";

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

<RoadLayerControls name="Core network" style="cn">
  <button class="outline" on:click={importExisting}>Import core network</button>

  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={showTruth} />
      Show actual core network
    </label>

    <label>
      <input type="checkbox" bind:checked={showMatched} />
      Show map-matched core network
    </label>
  {/if}
</RoadLayerControls>

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
