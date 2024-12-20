<script lang="ts">
  import { downloadGeneratedFile, Modal } from "svelte-utils";
  import {
    backend,
    boundaryName,
    currentFilename,
    mainModeRoutesChanged,
  } from "../stores";
  import { getKey, listFilesInBoundary } from "./files";
  import Link from "./Link.svelte";

  let open = false;
  let fileList = listFilesInBoundary($boundaryName);

  async function rename() {
    let oldName = $currentFilename;
    let newName = window.prompt(`Rename file ${oldName} to what?`, oldName);
    // TODO Confirm overwriting
    if (newName) {
      let value = await $backend!.toSavefile();
      window.localStorage.setItem(getKey($boundaryName, newName), value);
      window.localStorage.removeItem(getKey($boundaryName, oldName));
      $currentFilename = newName;
      fileList = listFilesInBoundary($boundaryName);
    }
  }

  async function makeCopy() {
    let oldName = $currentFilename;
    let newName = window.prompt(
      `Make a copy of file ${oldName} as what name?`,
      `${oldName} v2`,
    );
    // TODO Confirm overwriting
    if (newName) {
      let value = await $backend!.toSavefile();
      window.localStorage.setItem(getKey($boundaryName, newName), value);
      $currentFilename = newName;
      fileList = listFilesInBoundary($boundaryName);
    }
  }

  async function exportFile() {
    let file = `npw_${$boundaryName}_${$currentFilename}.geojson`;
    downloadGeneratedFile(file, await $backend!.toSavefile());
  }

  async function openFile(filename: string) {
    let value = window.localStorage.getItem(getKey($boundaryName, filename));
    if (value) {
      try {
        await $backend!.loadSavefile(value);
        $currentFilename = filename;
        open = false;
        $mainModeRoutesChanged += 1;
      } catch (err) {
        window.alert(`Couldn't restore saved state: ${err}`);
      }
    }
  }

  async function deleteFile() {
    if (
      !window.confirm(`Really delete ${$currentFilename}? You can't undo this`)
    ) {
      return;
    }
    window.localStorage.removeItem(getKey($boundaryName, $currentFilename));
    fileList = listFilesInBoundary($boundaryName);
    await openFile(fileList[0][0]);
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
      <button class="secondary" on:click={makeCopy}>Make copy</button>
      <button class="secondary" on:click={exportFile}>Export</button>
      <button
        class="warning"
        on:click={deleteFile}
        disabled={fileList.length < 2}
      >
        Delete
      </button>
    </div>

    <hr />

    <p>Load a different file in {$boundaryName}:</p>
    <ul>
      {#each fileList as [filename, description]}
        {#if filename == $currentFilename}
          <li>{filename} (currently open)</li>
        {:else}
          <li>
            <Link on:click={() => openFile(filename)}>
              {filename} ({description})
            </Link>
          </li>
        {/if}
      {/each}
    </ul>

    <button on:click={() => (open = false)}>OK</button>
  </Modal>
{/if}
