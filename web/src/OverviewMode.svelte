<script lang="ts">
  import { SplitComponent } from "./common/layout";
  import ManageFiles from "./common/ManageFiles.svelte";
  import { changeStage, currentStage } from "./stores";

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

      <div><a href="index.html">Pick area</a></div>

      <div><ManageFiles /></div>

      <p>
        Start by planning the primary and secondary tiers, before adding access
        to local places
      </p>

      {#each Object.entries(tiers) as [stage, label]}
        <div>
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
    </div>
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
