<script lang="ts">
  import {
    VectorTileSource,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import {
    backend,
    infraTypeMapping,
    autosave,
    mainModeRoutesChanged,
    assetUrl,
  } from "../stores";
  import { colorByInfraType } from "../colors";
  import { Popup, constructMatchExpression } from "svelte-utils/map";
  import { Modal } from "svelte-utils";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let firstLoad = false;
  let showImportModal = false;

  $: if (show) {
    firstLoad = true;
  }

  async function importExisting() {
    showImportModal = false;
    if ($backend) {
      let numChanges = $backend.importExistingRoutes();
      let noun = numChanges == 1 ? "route segment" : "route segments";
      await autosave();
      $mainModeRoutesChanged += 1;
      window.alert(`Imported ${numChanges} ${noun}`);
      show = false;
    }
  }

  let showTruth = true;
  let showCalculated = true;
</script>

<LayerControls name="existing network">
  <label>
    <input type="checkbox" bind:checked={show} />
    Existing network
  </label>

  {#if show}
    <button class="outline" on:click={() => (showImportModal = true)}>
      Import existing routes
    </button>

    <label>
      <input type="checkbox" bind:checked={showTruth} />
      Show osmactive data
    </label>

    <label>
      <input type="checkbox" bind:checked={showCalculated} />
      Show calculated data
    </label>
  {/if}
</LayerControls>

{#if showImportModal}
  <Modal on:close={() => (showImportModal = false)}>
    <h2>Import existing routes from OpenStreetMap</h2>

    <p>
      This will add existing routes to your network. Only Segregated Tracks
      (wide and narrow) and Off Road Cycleways will be imported; other
      infrastructure types are not high-quality enough in practice to be
      considered part of the network. OpenStreetMap data and this tool's
      interpretation of it are imperfect, so please check the results carefully
      and remove / adjust any errors.

      <button on:click={importExisting}>Import existing routes</button>
      <button class="secondary" on:click={() => (showImportModal = false)}>
        Cancel
      </button>
    </p>
  </Modal>
{/if}

<!-- TODO For debugging -->
<VectorTileSource url={`pmtiles://${assetUrl("cbd.pmtiles")}`}>
  <LineLayer
    sourceLayer="cbd_layer"
    filter={["!=", ["get", "Infrastructure type"], "Mixed Traffic Street"]}
    paint={{
      "line-color": constructMatchExpression(
        ["get", "Infrastructure type"],
        {
          "Segregated Track (wide)": "#054d05",
          "Off Road Cycleway": "#3a9120",
          "Segregated Track (narrow)": "#87d668",
          "Shared Footway": "#ffbf00",
          "Painted Cycle Lane": "#FF0000",
        },
        "cyan",
      ),
      "line-width": 3,
    }}
    layout={{
      visibility: show && showTruth ? "visible" : "none",
    }}
  />
</VectorTileSource>

{#if $backend && firstLoad}
  {#await $backend.classifyExistingNetwork() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        layout={{
          visibility: show && showCalculated ? "visible" : "none",
        }}
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": colorByInfraType,
          "line-opacity": 0.8,
        }}
        manageHoverState
        on:click={(e) =>
          window.open(e.detail.features[0].properties.way, "_blank")}
        hoverCursor="pointer"
      >
        <Popup openOn="hover" let:props>
          {infraTypeMapping[props.infra_type][0]}
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
