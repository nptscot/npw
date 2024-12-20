<script lang="ts">
  import { downloadGeneratedFile, Modal } from "svelte-utils";
  import { backend, boundaryName, currentFilename } from "../stores";
  import { getKey } from "./files";

  let open = false;

  async function rename() {
    let oldName = $currentFilename;
    let newName = window.prompt(`Rename file ${oldName} to what?`, oldName);
    // TODO Confirm overwriting
    if (newName) {
      let value = await $backend!.toSavefile();
      window.localStorage.setItem(getKey($boundaryName, newName), value);
      window.localStorage.removeItem(getKey($boundaryName, oldName));
      $currentFilename = newName;
    }
  }

  async function exportFile() {
    let file = `npw_${$boundaryName}_${$currentFilename}.geojson`;
    downloadGeneratedFile(file, await $backend!.toSavefile());
  }
</script>

<button class="secondary" on:click={() => (open = true)}>
  Manage files: {$currentFilename}
</button>

{#if open}
  <Modal on:close={() => (open = false)}>
    <h2>Manage files</h2>

    <p>
      You're currently editing: <u>{$currentFilename}</u>
    </p>
    <div role="group">
      <button class="secondary" on:click={rename}>Rename</button>
      <button class="secondary" on:click={exportFile}>Export</button>
    </div>

    <button on:click={() => (open = false)}>OK</button>
  </Modal>
{/if}
