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
    referenceRoadStyle,
  } from "./stores";

  type Subpage =
    | "overview"
    | "report"
    | "disconnected"
    | "mesh-density"
    | "streetspace";
  let subpage: Subpage = "overview";

  let loading = "";
  // TODO Will this state get lost as the user navigates around? Seemingly no,
  // because its only used in a stage that doesn't have much editing.
  let lastUpdateOD = 0;

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

  function changePage(page: Subpage) {
    $gridMeshDensity = false;
    $referenceRoadStyle = "off";

    subpage = page;

    if (subpage == "disconnected") {
      $referenceRoadStyle = "disconnections";
    } else if (subpage == "mesh-density") {
      $gridMeshDensity = true;
    } else if (subpage == "streetspace") {
      $referenceRoadStyle = "deliverability";
    }
  }
</script>

<Loading {loading} />

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      {#if subpage == "overview"}
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
          Having designed your network, you can now assess its performance and
          fix any problems.
        </p>

        <h4>Metrics</h4>

        <button class="ds_button" on:click={() => changePage("report")}>
          Show network metrics
        </button>

        <h4>Disconnections</h4>

        <p>TODO show number of components</p>
        <button class="ds_button" on:click={() => changePage("disconnected")}>
          Identify disconnected areas
        </button>

        <h4>Coverage</h4>

        <button class="ds_button" on:click={() => changePage("mesh-density")}>
          Analyse mesh density
        </button>

        <h4>Deliverability</h4>

        <p>TODO show percent</p>
        <button class="ds_button" on:click={() => changePage("streetspace")}>
          Check routes that do not fit in available streetspace
        </button>

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
            Initial stats calculation disabled for faster start. Press the
            button above.
          </p>
        {/if}
      {:else if subpage == "report"}
        <header class="ds_page-header">
          <h2 class="ds_page-header__title">Network metrics</h2>
        </header>

        <div>
          <button
            type="button"
            class="ds_link"
            on:click={() => changePage("overview")}
          >
            <i class="fa-solid fa-chevron-left"></i>
            Back to network assessment
          </button>
        </div>

        <p>The network you have designed performs as follows:</p>

        <FinalReport />
      {:else if subpage == "disconnected"}
        <header class="ds_page-header">
          <h2 class="ds_page-header__title">Network disconnections</h2>
        </header>

        <div>
          <button
            type="button"
            class="ds_link"
            on:click={() => changePage("overview")}
          >
            <i class="fa-solid fa-chevron-left"></i>
            Back to network assessment
          </button>
        </div>

        TODO
      {:else if subpage == "mesh-density"}
        <header class="ds_page-header">
          <h2 class="ds_page-header__title">Network coverage</h2>
        </header>

        <div>
          <button
            type="button"
            class="ds_link"
            on:click={() => changePage("overview")}
          >
            <i class="fa-solid fa-chevron-left"></i>
            Back to network assessment
          </button>
        </div>

        TODO
      {:else if subpage == "streetspace"}
        <header class="ds_page-header">
          <h2 class="ds_page-header__title">Streetspace deliverability</h2>
        </header>

        <div>
          <button
            type="button"
            class="ds_link"
            on:click={() => changePage("overview")}
          >
            <i class="fa-solid fa-chevron-left"></i>
            Back to network assessment
          </button>
        </div>
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
