<script lang="ts">
  import { Loading, QualitativeLegend } from "svelte-utils";
  import { cnTierColors } from "../../colors";
  import { autosave, backend, devMode, referenceRoadStyle } from "../../stores";
  import { debugOriginalData } from "../stores";

  let loading = "";

  async function importExisting() {
    if ($backend) {
      loading = "Importing core network";
      let numChanges = await $backend.importCoreNetwork();
      loading = "";
      let noun = numChanges == 1 ? "route segment" : "route segments";
      await autosave();
      window.alert(`Imported ${numChanges} ${noun}`);
      $referenceRoadStyle = "off";
    }
  }
</script>

<Loading {loading} />

{#if $referenceRoadStyle == "cn"}
  <QualitativeLegend colors={cnTierColors} horiz />

  <button class="outline" on:click={importExisting}>Import core network</button>

  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={$debugOriginalData} />
      Show original data
    </label>
  {/if}
{/if}
