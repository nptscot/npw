<script lang="ts">
  import { downloadGeneratedFile } from "svelte-utils";
  import { Link, stripPrefix } from "./common";
  import { getKey, listFilesInBoundary, setLocalStorage } from "./common/files";
  import { SplitComponent } from "./common/layout";
  import {
    autosave,
    backend,
    boundaryName,
    currentFilename,
    mode,
    mutationCounter,
  } from "./stores";

  export let subpage: "explore" | "project-list";

  let fileList = listFilesInBoundary($boundaryName);

  function prettyprintBoundary() {
    return stripPrefix($boundaryName, "LAD_");
  }

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
    $mode = { kind: "overview" };
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
        $mutationCounter += 1;
        $mode = { kind: "overview" };
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
    $mode = { kind: "overview" };
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
        $mutationCounter += 1;
        $mode = { kind: "overview" };
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

<SplitComponent>
  <div slot="controls" class="main-controls">
    {#if subpage == "explore"}
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Explore {prettyprintBoundary()}</h2>
      </header>

      <div>
        <button
          type="button"
          class="ds_link"
          on:click={() => (window.location.href = "./html")}
        >
          <i class="fa-solid fa-chevron-left"></i>
          Work in a different area
        </button>
      </div>

      <p>Explore the map and layers, or begin a new network project.</p>
      <p>
        You should familiarise yourself with current infrastructure, via the
        layers on the right, before starting your new cycle network.
      </p>

      <div>
        <button
          class="ds_button"
          on:click={() => ($mode = { kind: "setup", subpage: "project-list" })}
        >
          Design my network
        </button>
      </div>
    {:else if subpage == "project-list"}
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Design my network</h2>
      </header>

      <div>
        <button
          type="button"
          class="ds_link"
          on:click={() => ($mode = { kind: "setup", subpage: "explore" })}
        >
          <i class="fa-solid fa-chevron-left"></i>
          Back
        </button>
      </div>

      <p>Start a new project for this area or choose a previous project.</p>

      <button
        class="ds_button"
        disabled={!$currentFilename}
        on:click={() => ($mode = { kind: "overview" })}
      >
        Start
      </button>

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
    {/if}
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
