<script lang="ts">
  import { Loading } from "svelte-utils";
  import { BackLink } from "../common";
  import { SplitComponent } from "../common/layout";
  import {
    backend,
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

  let loading = "";

  async function checkDirectness() {
    if (!$slowStats || $lastUpdateSlowStats != $mutationCounter) {
      loading = "Recalculating directness";
      $slowStats = await $backend!.recalculateSlowStats();
      $lastUpdateSlowStats = $mutationCounter;
      loading = "";
    }
    $mode = {
      kind: "evaluate-journey",
      browse: $slowStats.worst_directness_routes,
    };
  }
</script>

<Loading {loading} />

<SplitComponent>
  <svelte:fragment slot="controls">
    <div class="main-controls">
      {#if $subpage == "overview"}
        <header class="ds_page-header">
          <h2 class="ds_page-header__title">Assess the new network</h2>
        </header>

        <BackLink on:click={() => ($mode = { kind: "overview" })}>
          Back to project overview
        </BackLink>

        <p>
          Having designed your network, you can now fix any problems and assess
          its performance.
        </p>

        <h3>Problems</h3>

        <div>
          <button
            class="ds_button"
            on:click={() => ($subpage = "disconnected")}
          >
            Fix network splits
          </button>
        </div>

        <div>
          <button
            class="ds_button"
            on:click={() => ($subpage = "mesh-density")}
          >
            Check mesh density
          </button>
        </div>

        <div>
          <button class="ds_button" on:click={() => ($subpage = "population")}>
            Fix unreachable population zones
          </button>
        </div>

        <div>
          <button class="ds_button" on:click={() => ($subpage = "streetspace")}>
            Fix infrastructure that does not fit
          </button>
        </div>

        <h3>Check overall performance</h3>

        <div>
          <button class="ds_button" on:click={() => ($subpage = "report")}>
            Show network metrics
          </button>
        </div>

        <div>
          <button
            class="ds_button"
            on:click={() => ($subpage = "calculated-routes")}
          >
            See network impacts on demand
          </button>
        </div>

        <div>
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
        </div>

        <div>
          <button class="ds_button" on:click={checkDirectness}>
            Check journeys used to calculate directness
          </button>
        </div>
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
