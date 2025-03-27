<script lang="ts">
  import { Loading, notNull } from "svelte-utils";
  import { SplitComponent } from "./common/layout";
  import { gridMeshDensity } from "./layers/stores";
  import FinalReport from "./stats/FinalReport.svelte";
  import ODBreakdowns from "./stats/ODBreakdowns.svelte";
  import {
    backend,
    exitCurrentStage,
    mode,
    mutationCounter,
    odStats,
  } from "./stores";

  let loading = "";
  // TODO Will this state get lost as the user navigates around? Seemingly no,
  // because its only used in a stage that doesn't have much editing.
  let lastUpdateOD = 0;

  let showFinalReport = false;

  async function recalcOD() {
    loading = "Recalculating OD stats";
    $odStats = await $backend!.recalculateODStats();
    loading = "";
    lastUpdateOD = $mutationCounter;
  }

  function gotoOverview() {
    exitCurrentStage();
    $mode = { kind: "overview" };
  }
</script>

<Loading {loading} />

<SplitComponent>
  <div slot="controls">
    <FinalReport bind:show={showFinalReport} />

    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Assess the new network</h2>
      </header>

      <div>
        <button type="button" class="ds_link" on:click={gotoOverview}>
          <i class="fa-solid fa-chevron-left"></i>
          Back to project overview
        </button>
      </div>

      <p>
        Having designed your network, you can now assess its performance and fix
        any problems.
      </p>

      <label>
        <input type="checkbox" bind:checked={$gridMeshDensity} />
        Mesh density
      </label>

      <p>Streetspace</p>

      <p>Disconnections</p>

      <div>
        <button
          class="ds_button ds_button--secondary"
          on:click={() => (showFinalReport = true)}
        >
          Final report
        </button>
      </div>

      <div>
        <button
          class="ds_button ds_button--secondary"
          on:click={recalcOD}
          disabled={$mutationCounter == lastUpdateOD}
        >
          Recalculate route network
        </button>
      </div>

      {#if $odStats}
        <div style:margin-top="4px" style:border="2px solid black">
          <p>
            <!-- svelte-ignore a11y-invalid-attribute -->
            <a
              href="#"
              on:click|preventDefault={() =>
                ($mode = {
                  kind: "evaluate-journey",
                  browse: notNull($odStats).worst_directness_routes,
                })}
            >
              Average weighted directness
            </a>
            : {$odStats.average_weighted_directness.toFixed(1)}x
          </p>

          <ODBreakdowns od={$odStats} />
        </div>
      {:else}
        <p>
          Initial stats calculation disabled for faster start. Press the button
          above.
        </p>
      {/if}
    </div>
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
