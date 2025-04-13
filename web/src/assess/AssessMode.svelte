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

        <h4>Problems</h4>

        <button class="ds_button" on:click={() => ($subpage = "disconnected")}>
          Fix network splits
        </button>

        <button class="ds_button" on:click={() => ($subpage = "mesh-density")}>
          Check mesh density
        </button>

        <button class="ds_button" on:click={() => ($subpage = "population")}>
          Fix unreachable population zones
        </button>

        <button class="ds_button" on:click={() => ($subpage = "streetspace")}>
          Fix routes that do not fit
        </button>

        <h4>Check overall performance</h4>

        <div>
          <button class="ds_button" on:click={() => ($subpage = "report")}>
            Show network metrics
          </button>
        </div>

        <button
          class="ds_button"
          on:click={() => ($subpage = "calculated-routes")}
        >
          See network impacts on demand
        </button>

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
