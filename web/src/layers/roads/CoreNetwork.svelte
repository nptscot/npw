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
  <QualitativeLegend colors={cnTierColors} />

  <br />

  <div><button on:click={importExisting}>Import core network</button></div>

  {#if $devMode}
    <br />

    <label>
      <input type="checkbox" bind:checked={$debugOriginalData} />
      Show original data
    </label>
  {/if}
{/if}
