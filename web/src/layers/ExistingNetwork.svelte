<script lang="ts">
  import { GeoJSON, LineLayer, VectorTileSource } from "svelte-maplibre";
  import { Loading, Modal, notNull } from "svelte-utils";
  import { constructMatchExpression, Popup } from "svelte-utils/map";
  import { colorByInfraType } from "../colors";
  import { layerId, roadLineWidth } from "../common";
  import { assetUrl, autosave, backend, infraTypeMapping } from "../stores";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let firstLoad = false;
  let showImportModal = false;
  let loading = "";

  $: if (show) {
    firstLoad = true;
  }

  async function importExisting() {
    showImportModal = false;
    if ($backend) {
      loading = "Importing existing routes from OSM";
      let numChanges = await $backend.importExistingRoutes();
      loading = "";
      let noun = numChanges == 1 ? "route segment" : "route segments";
      await autosave();
      window.alert(`Imported ${numChanges} ${noun}`);
      show = false;
    }
  }

  let showTruth = false;
  let showCalculated = true;
</script>

<Loading {loading} />

<LayerControls name="Existing network" bind:show>
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
    {...layerId("existing-infra-debug")}
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
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility: show && showTruth ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      osmactive: {props["Infrastructure type"]}
    </Popup>
  </LineLayer>
</VectorTileSource>

{#if $backend && firstLoad}
  {#await $backend.classifyExistingNetwork() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("existing-infra")}
        layout={{
          visibility: show && showCalculated ? "visible" : "none",
        }}
        paint={{
          "line-width": roadLineWidth(0),
          "line-color": colorByInfraType,
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
