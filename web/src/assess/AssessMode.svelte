<script lang="ts">
  import { SplitComponent } from "../common/layout";
  import {
    lastUpdateSlowStats,
    mode,
    mutationCounter,
    slowStats,
  } from "../stores";
  import Disconnections from "./Disconnections.svelte";
  import FinalReport from "./FinalReport.svelte";
  import { subpage } from "./index";
  import MeshDensity from "./MeshDensity.svelte";
  import ODBreakdowns from "./ODBreakdowns.svelte";
  import Population from "./Population.svelte";
  import Streetspace from "./Streetspace.svelte";
</script>

<SplitComponent>
  <svelte:fragment slot="controls">
    <div class="main-controls">
      {#if $subpage == "overview"}
        <header class="ds_page-header">
          <h2 class="ds_page-header__title">Assess the new network</h2>
        </header>

        <div>
          <button
            type="button"
            class="ds_link"
            on:click={() => ($mode = { kind: "overview" })}
          >
            <i class="fa-solid fa-chevron-left"></i>
            Back to project overview
          </button>
        </div>

        <p>
          Having designed your network, you can now assess its performance and
          fix any problems.
        </p>

        <h4>Disconnections</h4>

        <button class="ds_button" on:click={() => ($subpage = "disconnected")}>
          Identify network splits
        </button>

        <h4>Coverage</h4>

        <button class="ds_button" on:click={() => ($subpage = "mesh-density")}>
          Analyse mesh density
        </button>

        <button class="ds_button" on:click={() => ($subpage = "population")}>
          Identify population zones without coverage
        </button>

        <h4>Deliverability</h4>

        <button class="ds_button" on:click={() => ($subpage = "streetspace")}>
          Check routes that do not fit in available streetspace
        </button>

        <hr />

        <h4>Metrics</h4>

        <div>
          <button class="ds_button" on:click={() => ($subpage = "report")}>
            Show network metrics
          </button>
        </div>

        <h4>Effects on demand</h4>

        <button
          class="ds_button"
          on:click={() => ($subpage = "calculated-routes")}
        >
          See how the demand uses the network
        </button>

        <h4>Evaluate a journey</h4>

        <button
          class="ds_button"
          on:click={() =>
            ($mode = {
              kind: "evaluate-journey",
              browse: [],
            })}
        >
          Evaluate a journey
        </button>

        {#if $slowStats && $lastUpdateSlowStats == $mutationCounter}
          <button
            class="ds_button"
            on:click={() =>
              ($mode = {
                kind: "evaluate-journey",
                browse: $slowStats.worst_directness_routes,
              })}
          >
            Check journeys used to calculate directness
          </button>
        {/if}
      {:else if $subpage == "report"}
        <FinalReport />
      {:else if $subpage == "disconnected"}
        <Disconnections />
      {:else if $subpage == "mesh-density"}
        <MeshDensity />
      {:else if $subpage == "streetspace"}
        <Streetspace />
      {:else if $subpage == "population"}
        <Population />
      {:else if $subpage == "calculated-routes"}
        <ODBreakdowns />
      {/if}
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
