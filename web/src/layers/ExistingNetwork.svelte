<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import {
    backend,
    infraTypeMapping,
    autosave,
    mainModeRoutesChanged,
  } from "../stores";
  import { colorByInfraType } from "../colors";
  import { Popup } from "svelte-utils/map";
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

{#if $backend && firstLoad}
  {#await $backend.classifyExistingNetwork() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        layout={{
          visibility: show ? "visible" : "none",
        }}
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": colorByInfraType,
          "line-opacity": 0.8,
        }}
        manageHoverState
      >
        <Popup openOn="hover" let:props>
          {infraTypeMapping[props.infra_type][0]}
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
