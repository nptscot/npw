<script lang="ts">
  import { Loading } from "svelte-utils";
  import { Checkbox } from "./common";
  import { listFilesInBoundary } from "./common/files";
  import {
    autosave,
    backend,
    boundaryName,
    mode,
    mutationCounter,
    setCurrentFile,
  } from "./stores";

  export let subpage: "explore" | "project-list" | "new-project";

  let name = "";
  let usedFiles = new Set(
    listFilesInBoundary($boundaryName).map(([filename, _]) => filename),
  );

  let loading = "";
  let useCN = false;
  let useExistingSomeTypes = false;
  let useExistingHighLoS = false;
  $: reimport(useCN, useExistingSomeTypes, useExistingHighLoS);

  async function newFile() {
    setCurrentFile(name);
    // Immediately save the new file
    await autosave();
    $mode = { kind: "overview" };
  }

  async function reimport(
    useCN: boolean,
    useExistingSomeTypes: boolean,
    useExistingHighLoS: boolean,
  ) {
    loading = "Loading network for preview";
    await $backend!.clearAllRoutes();

    if (useCN) {
      await $backend!.importCoreNetwork();
    }
    if (useExistingSomeTypes) {
      await $backend!.importExistingRoutes("infra-type");
    }
    if (useExistingHighLoS) {
      await $backend!.importExistingRoutes("los");
    }

    $mutationCounter += 1;
    loading = "";
  }
</script>

<Loading {loading} />

<header class="ds_page-header">
  <h2 class="ds_page-header__title">New network design</h2>
</header>

<div>
  <button
    type="button"
    class="ds_link"
    on:click={() => (subpage = "project-list")}
  >
    <i class="fa-solid fa-chevron-left"></i>
    Back
  </button>
</div>

<p>
  The NPT provides a set of starting points for the network, or you can start
  from a blank map.
</p>

<p>Enter a title for the network.</p>

<input
  class="ds_input ds_input--fixed-20"
  placeholder="Name"
  bind:value={name}
/>
{#if usedFiles.has(name)}
  <p class="ds_question__error-message">
    <span class="visually-hidden">Error:</span>
    There's already a project with this name; choose another
  </p>
{/if}

<button
  class="ds_button"
  disabled={name.length == 0 ||
    usedFiles.has(name) ||
    useExistingHighLoS ||
    useExistingSomeTypes ||
    useCN}
  on:click={newFile}
>
  Start designing from blank map
</button>

<h4>Use existing layers</h4>

<div class="ds_field-group">
  <Checkbox bind:checked={useExistingHighLoS}>
    All existing infrastructure from OpenStreetMap, if it achieves high Level of
    Service
  </Checkbox>

  <Checkbox bind:checked={useExistingSomeTypes}>
    Existing segregated tracks and off-road cycleways from OpenStreetMap
  </Checkbox>

  <Checkbox bind:checked={useCN}>Core network</Checkbox>
</div>

<!--<p>Or clone from your existing projects</p>-->

<br />

<button
  class="ds_button"
  disabled={name.length == 0 ||
    usedFiles.has(name) ||
    !(useExistingSomeTypes || useExistingHighLoS || useCN)}
  on:click={newFile}
>
  Start designing from existing layers
</button>
