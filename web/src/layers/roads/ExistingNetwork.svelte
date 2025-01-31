<script lang="ts">
  import { Loading, Modal } from "svelte-utils";
  import { autosave, backend, devMode, referenceRoadStyle } from "../../stores";
  import { debugOriginalData } from "./stores";

  let showImportModal = false;
  let loading = "";

  async function importExisting(kind: "infra-type" | "los") {
    showImportModal = false;
    if ($backend) {
      loading = "Importing existing routes from OSM";
      let numChanges = await $backend.importExistingRoutes(kind);
      loading = "";
      let noun = numChanges == 1 ? "route segment" : "route segments";
      await autosave();
      window.alert(`Imported ${numChanges} ${noun}`);
      $referenceRoadStyle = "off";
    }
  }
</script>

<Loading {loading} />

<button class="outline" on:click={() => (showImportModal = true)}>
  Import existing routes
</button>

{#if $devMode}
  <label>
    <input type="checkbox" bind:checked={$debugOriginalData} />
    Show osmactive data
  </label>
{/if}

{#if showImportModal}
  <span class="pico">
    <Modal on:close={() => (showImportModal = false)}>
      <h2>Import existing routes from OpenStreetMap</h2>

      <p>
        This will add existing routes to your network. OpenStreetMap data and
        this tool's interpretation of it are imperfect, so please check the
        results carefully and remove / adjust any errors.
      </p>
      <p>
        You can choose to import only Segregated Tracks (wide and narrow) and
        Off Road Cycleways, or any infrastructure that achieves a high level of
        service. For the latter option, if the result shows Painted Cycle Lanes,
        then they are adequate to improve the level of service given the
        estimated speed and traffic volumes.
      </p>

      <button on:click={() => importExisting("infra-type")}>
        Import existing Segregated Tracks and Off Road Cycleways
      </button>
      <br />
      <button on:click={() => importExisting("los")}>
        Import existing routes that achieve a high level of service
      </button>
      <br />
      <button class="secondary" on:click={() => (showImportModal = false)}>
        Cancel
      </button>
    </Modal>
  </span>
{/if}
