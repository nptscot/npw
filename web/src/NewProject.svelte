<script lang="ts">
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

  let useExistingSomeTypes = false;
  let useExistingHighLoS = false;
  let useArterialRoads = false;
  $: reimport(useExistingSomeTypes, useExistingHighLoS, useArterialRoads);

  async function newFile() {
    setCurrentFile(name);
    // Immediately save the new file
    await autosave();
    $mode = { kind: "overview" };
  }

  async function reimport(
    useExistingSomeTypes: boolean,
    useExistingHighLoS: boolean,
    useArterialRoads: boolean,
  ) {
    await $backend!.clearAllRoutes();

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
  }
</script>

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
  required
/>
{#if usedFiles.has(name)}
  <p class="ds_question__error-message">
    <span class="visually-hidden">Error:</span>
    There's already a project with this name; choose another
  </p>
{/if}

<p>
  You can optionally choose to include one or more of these starting points:
</p>

<div class="ds_field-group">
  <Checkbox bind:checked={useExistingHighLoS}>
    All existing infrastructure, if it achieves high Level of Service
  </Checkbox>

  <Checkbox bind:checked={useExistingSomeTypes}>
    Existing segregated tracks and off-road cycleways
  </Checkbox>

  <Checkbox bind:checked={useArterialRoads}>
    All arterial roads (trunk, A, B, and C roads)
  </Checkbox>
</div>

<button
  class="ds_button"
  disabled={name.length == 0 || usedFiles.has(name)}
  on:click={newFile}
>
  Start
</button>

<p><i>These base networks are based on OpenStreetMap data.</i></p>

<!--<p>Or clone from your existing projects</p>-->
