<script lang="ts">
  import { tierColors } from "./colors";
  import { SplitComponent } from "./common/layout";
  import { changeStage, currentFilename, currentStage, mode } from "./stores";
  import type { Tier } from "./types";

  let tiers = {
    Primary: "Design primary routes",
    Secondary: "Design secondary routes",
    LocalAccess: "Design local access to POIs",
    LongDistance: "Design long distance routes",
  } as { [name in Tier]: string };

  // TODO Get TS for Object.entries better
  function castTier(x: string): Tier {
    return x as Tier;
  }
</script>

<SplitComponent>
  <svelte:fragment slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Plan my network</h2>
      </header>

      <div>
        <button
          type="button"
          class="ds_link"
          on:click={() => ($mode = { kind: "setup", subpage: "project-list" })}
        >
          <i class="fa-solid fa-chevron-left"></i>
          Close project ({$currentFilename})
        </button>
      </div>

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
        Assess performance of the network
      </button>

      <h4>Export project</h4>

      <button
        class="ds_button ds_button--secondary"
        on:click={() => ($mode = { kind: "export" })}
      >
        Export project
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
