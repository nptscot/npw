<script lang="ts">
  import { Loading } from "svelte-utils";
  import { BackLink, Checkbox } from "./common";
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
  let useArterialRoads = false;
  $: reimport(
    useCN,
    useExistingSomeTypes,
    useExistingHighLoS,
    useArterialRoads,
  );

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
    useArterialRoads: boolean,
  ) {
    loading = "Loading network for preview";
    await $backend!.clearAllRoutes();

    if (useCN) {
      await $backend!.importCoherentNetwork();
    }
    if (useExistingSomeTypes) {
      await $backend!.importExistingRoutes("infra-type");
    }
    if (useExistingHighLoS) {
      await $backend!.importExistingRoutes("los");
    }
    if (useArterialRoads) {
      await $backend!.importArterialRoads();
    }

    $mutationCounter += 1;
    loading = "";
  }
</script>

<Loading {loading} />

<header class="ds_page-header">
  <h2 class="ds_page-header__title">New network design</h2>
</header>

<BackLink on:click={() => (subpage = "project-list")}>Back</BackLink>

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

<h4>Plan a network from scratch</h4>

<button
  class="ds_button"
  disabled={name.length == 0 ||
    usedFiles.has(name) ||
    useExistingHighLoS ||
    useExistingSomeTypes ||
    useCN}
  on:click={newFile}
>
  Start
</button>

<h4>Plan a network from existing infrastructure</h4>

<div class="ds_field-group">
  <Checkbox bind:checked={useExistingHighLoS}>
    All existing infrastructure from OpenStreetMap, if it achieves high Level of
    Service
  </Checkbox>

  <Checkbox bind:checked={useExistingSomeTypes}>
    Existing segregated tracks and off-road cycleways from OpenStreetMap
  </Checkbox>
</div>

<br />

<button
  class="ds_button"
  disabled={name.length == 0 ||
    usedFiles.has(name) ||
    !(useExistingSomeTypes || useExistingHighLoS)}
  on:click={newFile}
>
  Start
</button>

<h4>Plan a network using shortcuts</h4>

<div class="ds_field-group">
  <Checkbox bind:checked={useCN}>Coherent network</Checkbox>

  <Checkbox bind:checked={useArterialRoads}>All arterial roads</Checkbox>
</div>

<br />

<button
  class="ds_button"
  disabled={name.length == 0 ||
    usedFiles.has(name) ||
    !(useCN || useArterialRoads)}
  on:click={newFile}
>
  Start
</button>

<!--<p>Or clone from your existing projects</p>-->
