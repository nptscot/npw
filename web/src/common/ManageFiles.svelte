<script lang="ts">
  import { downloadGeneratedFile } from "svelte-utils";
  import {
    autosave,
    backend,
    boundaryName,
    currentFilename,
    mutationCounter,
  } from "../stores";
  import { getKey, listFilesInBoundary, setLocalStorage } from "./files";
  import { Link, Modal } from "./index";

  let open = false;
  let fileList = listFilesInBoundary($boundaryName);

  async function rename() {
    let oldName = $currentFilename;
    let newName = window.prompt(`Rename file ${oldName} to what?`, oldName);
    // TODO Confirm overwriting
    if (newName) {
      let value = await $backend!.toSavefile();
      setLocalStorage(getKey($boundaryName, newName), value);
      window.localStorage.removeItem(getKey($boundaryName, oldName));
      setCurrentFile(newName);
      fileList = listFilesInBoundary($boundaryName);
    }
  }

  async function newFile() {
    let name = window.prompt(`What do you want to name the new file?`);
    // TODO Confirm overwriting
    if (!name) {
      return;
    }
    await $backend!.clearAllRoutes();
    $currentFilename = name;
    $mutationCounter += 1;
    open = false;
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
      setLocalStorage(getKey($boundaryName, newName), value);
      setCurrentFile(newName);
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
        setCurrentFile(filename);
        open = false;
        $mutationCounter += 1;
      } catch (err) {
        window.alert(`Couldn't restore saved state: ${err}`);
      }
    }
  }

  async function clearFile() {
    if (
      !window.confirm(
        `Really clear all routes and reset ${$currentFilename} to a blank file? You can't undo this`,
      )
    ) {
      return;
    }
    await $backend!.clearAllRoutes();
    await autosave();
    open = false;
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

  let fileInput: HTMLInputElement;
  async function importFile(e: Event) {
    let rawFilename = fileInput.files![0].name;
    let value = await fileInput.files![0].text();

    try {
      await $backend!.loadSavefile(value);
    } catch (err) {
      window.alert(`This file doesn't seem to be valid: ${err}`);
      return;
    }

    let candidateFilename = guessFilename(rawFilename);
    while (true) {
      let newName = window.prompt(
        `What do you want to name this imported file?`,
        candidateFilename,
      );
      // TODO Confirm overwriting
      if (newName) {
        setLocalStorage(getKey($boundaryName, newName), value);
        fileList = listFilesInBoundary($boundaryName);
        setCurrentFile(newName);
        open = false;
        $mutationCounter += 1;
        return;
      }
    }
  }

  function guessFilename(filename: string): string {
    let prefix = `npw_${$boundaryName}_`;
    let suffix = ".geojson";
    if (filename.endsWith(suffix)) {
      filename = filename.slice(0, -suffix.length);
    }
    if (filename.startsWith(prefix)) {
      filename = filename.slice(prefix.length);
    }
    return filename;
  }

  // Updates the URL
  function setCurrentFile(name: string) {
    $currentFilename = name;

    let url = new URL(window.location.href);
    url.searchParams.set("file", name);
    window.history.replaceState(null, "", url.toString());
  }
</script>

<button on:click={() => (open = true)}>Pick project</button>

<Modal bind:show={open}>
  <h2>Manage files</h2>

  <p>
    You're currently editing: <u>{$currentFilename}</u>
  </p>
  <div class="ds_button-group">
    <button class="ds_button ds_button--secondary" on:click={rename}>
      Rename
    </button>
    <button class="ds_button ds_button--secondary" on:click={makeCopy}>
      Make copy
    </button>
    <button class="ds_button ds_button--secondary" on:click={exportFile}>
      Export
    </button>
    <button class="ds_button ds_button--secondary" on:click={clearFile}>
      Clear all routes
    </button>
    <button
      class="ds_button ds_button--secondary"
      on:click={deleteFile}
      disabled={fileList.length < 2}
    >
      Delete
    </button>
  </div>

  <hr />

  <button class="ds_button ds_button--secondary" on:click={newFile}>
    New file
  </button>

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

  <label>
    Load from an exported file
    <input bind:this={fileInput} on:change={importFile} type="file" />
  </label>

  <div>
    <button class="ds_button" on:click={() => (open = false)}>OK</button>
  </div>
</Modal>
