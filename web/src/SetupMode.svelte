<script lang="ts">
  import { onMount } from "svelte";
  import { stripPrefix } from "./common";
  import { getKey, listFilesInBoundary, setLocalStorage } from "./common/files";
  import { SplitComponent } from "./common/layout";
  import NewProject from "./NewProject.svelte";
  import {
    backend,
    boundaryName,
    currentFilename,
    mode,
    mutationCounter,
    setCurrentFile,
  } from "./stores";

  export let subpage: "explore" | "project-list" | "new-project";

  let fileList = listFilesInBoundary($boundaryName);

  // When entering this mode, clear out any state
  onMount(async () => {
    $currentFilename = "";
    await $backend!.clearAllRoutes();
    $mutationCounter += 1;

    let url = new URL(window.location.href);
    url.searchParams.delete("file");
    window.history.replaceState(null, "", url.toString());
  });

  function prettyprintBoundary() {
    return stripPrefix($boundaryName, "LAD_");
  }

  async function openFile(filename: string) {
    let value = window.localStorage.getItem(getKey($boundaryName, filename));
    if (value) {
      try {
        await $backend!.loadSavefile(value);
        setCurrentFile(filename);
      } catch (err) {
        window.alert(`Couldn't restore saved state: ${err}`);
      }
    }
  }

  function deleteFile(name: string) {
    if (!window.confirm(`Really delete ${name}? You can't undo this`)) {
      return;
    }
    window.localStorage.removeItem(getKey($boundaryName, name));
    fileList = listFilesInBoundary($boundaryName);
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
        setCurrentFile(newName);
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

      <h4>Start a new project for this area</h4>

      <button class="ds_button" on:click={() => (subpage = "new-project")}>
        New blank project
      </button>

      {#if fileList.length > 0}
        <h4>Or choose an existing project</h4>

        {#each fileList as [filename, description]}
          <div style="display: flex; justify-content: space-between;">
            <button class="ds_button" on:click={() => openFile(filename)}>
              {filename} ({description})
            </button>

            <button
              class="ds_button ds_button--secondary"
              on:click={() => deleteFile(filename)}
            >
              Delete
            </button>
          </div>
        {/each}
      {/if}

      <h4>Load from a file</h4>

      <label style:display="block">
        Load from an exported file
        <input bind:this={fileInput} on:change={importFile} type="file" />
      </label>
    {:else if subpage == "new-project"}
      <NewProject bind:subpage />
    {/if}
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
