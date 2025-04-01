<script lang="ts">
  import { tierColors } from "./colors";
  import { SplitComponent } from "./common/layout";
  import ManageFiles from "./common/ManageFiles.svelte";
  import { changeStage, currentStage, mode } from "./stores";

  let tiers = {
    Primary: "Design primary routes",
    Secondary: "Design secondary routes",
    LocalAccess: "Design local access to POIs",
    LongDistance: "Design long distance routes",
  };
</script>

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Plan my network</h2>
      </header>

      <div><a href="index.html">Work in a different area</a></div>

      <div><ManageFiles /></div>

      <p>
        Start by planning the primary and secondary tiers, before adding access
        to local places.
      </p>

      {#each Object.entries(tiers) as [stage, label]}
        <div
          style:border-left="5px solid {tierColors[stage]}"
          style:padding-left="8px"
        >
          <button
            class="ds_button"
            class:ds_button--secondary={$currentStage != stage}
            on:click={() => changeStage(stage)}
          >
            {label}
          </button>
        </div>
      {/each}

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
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
