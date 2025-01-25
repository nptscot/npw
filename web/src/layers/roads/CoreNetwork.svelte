<script lang="ts">
  import { Loading } from "svelte-utils";
  import {
    autosave,
    backend,
    devMode,
    roadStyle,
  } from "../../stores";
  import { debugOriginalData } from "./stores";

  let loading = "";

  async function importExisting() {
    if ($backend) {
      loading = "Importing core network";
      let numChanges = await $backend.importCoreNetwork();
      loading = "";
      let noun = numChanges == 1 ? "route segment" : "route segments";
      await autosave();
      window.alert(`Imported ${numChanges} ${noun}`);
            $roadStyle = "current_infra";
    }
  }
</script>

<Loading {loading} />

  <button class="outline" on:click={importExisting}>Import core network</button>

{#if $devMode}
  <label>
    <input type="checkbox" bind:checked={$debugOriginalData} />
    Show original data
  </label>
{/if}
