<script lang="ts">
  import { downloadGeneratedFile } from "svelte-utils";
  import { tierColors } from "./colors";
  import { BackLink } from "./common";
  import { SplitComponent } from "./common/layout";
  import {
    backend,
    boundaryName,
    changeStage,
    currentFilename,
    currentStage,
    mode,
  } from "./stores";
  import type { Tier } from "./types";

  let tiers = {
    Primary: "Plan primary routes",
    Secondary: "Plan secondary routes",
    LocalAccess: "Plan local access to POIs",
    LongDistance: "Plan long distance routes",
  } as { [name in Tier]: string };

  // TODO Get TS for Object.entries better
  function castTier(x: string): Tier {
    return x as Tier;
  }

  async function exportFile() {
    let file = `npw_${$boundaryName}_${$currentFilename}.geojson`;
    downloadGeneratedFile(file, JSON.stringify(await $backend!.getAllRoutes()));
  }
</script>

<SplitComponent>
  <svelte:fragment slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">NPW workflow overview</h2>
      </header>

      <BackLink
        on:click={() => ($mode = { kind: "setup", subpage: "project-list" })}
      >
        Close project ({$currentFilename})
      </BackLink>

      <p>
        Start by planning the primary and secondary tiers, before adding access
        to local places.
      </p>

      {#each Object.entries(tiers) as [stage, label]}
        <div
          style:border-left="5px solid {tierColors[castTier(stage)]}"
          style:padding-left="8px"
        >
          <button
            class="ds_button"
            class:ds_button--secondary={$currentStage != stage}
            style:margin-bottom="1rem"
            on:click={() => changeStage(stage)}
          >
            {label}
          </button>
        </div>
      {/each}
      <br />

      <h4>Assess my network</h4>

      <button
        class="ds_button ds_button--secondary"
        on:click={() => changeStage("assessment")}
      >
        Assess network quality
      </button>

      <h4>Export project</h4>

      <button class="ds_button ds_button--secondary" on:click={exportFile}>
        Export project file to share
      </button>
    </div>
  </svelte:fragment>
</SplitComponent>

<style>
  .main-controls {
    height: 100%;
    overflow-y: auto;
    padding: 20px;
  }
</style>
