<script lang="ts">
  import { Loading, Modal, QualitativeLegend } from "svelte-utils";
  import { infraTypeColors } from "../../colors";
  import { autosave, backend, devMode, referenceRoadStyle } from "../../stores";
  import { debugOriginalData } from "../stores";

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

{#if $referenceRoadStyle == "existing_infra"}
  <QualitativeLegend colors={infraTypeColors} />

  <button class="outline" on:click={() => (showImportModal = true)}>
    Import existing routes
  </button>

  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={$debugOriginalData} />
      Show osmactive data
    </label>
  {/if}
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

      <div style="display: flex; gap: 16px">
        <div style="width: 50%">
          <button on:click={() => importExisting("infra-type")}>
            Import existing Segregated Tracks and Off Road Cycleways
          </button>

          <p>These will all be imported, regardless of the level of service.</p>
        </div>

        <div style="width: 50%">
          <button on:click={() => importExisting("los")}>
            Import existing routes that achieve a high level of service
          </button>

          <p>
            If any existing infrastructure is adequate to achieve a high level
            of service, it will be imported. If this adds painted cycle lanes,
            then this means the speed and traffic volumes are low enough for
            this to be acceptable. Note shared footways are never imported.
          </p>
        </div>
      </div>

      <button class="secondary" on:click={() => (showImportModal = false)}>
        Cancel
      </button>
    </Modal>
  </span>
{/if}
