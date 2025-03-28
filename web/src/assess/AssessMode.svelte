<script lang="ts">
  import { SplitComponent } from "../common/layout";
  import FinalReport from "../stats/FinalReport.svelte";
  import ODBreakdowns from "../stats/ODBreakdowns.svelte";
  import { exitCurrentStage, mode } from "../stores";
  import Disconnections from "./Disconnections.svelte";
  import { changePage, subpage } from "./index";

  function gotoOverview() {
    exitCurrentStage();
    $mode = { kind: "overview" };
  }
</script>

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      {#if $subpage == "overview"}
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

        <button class="ds_button" on:click={() => changePage("disconnected")}>
          Identify disconnected areas
        </button>

        <h4>Coverage</h4>

        <button class="ds_button" on:click={() => changePage("mesh-density")}>
          Analyse mesh density
        </button>

        <h4>Deliverability</h4>

        <button class="ds_button" on:click={() => changePage("streetspace")}>
          Check routes that do not fit in available streetspace
        </button>

        <h4>Effects on demand</h4>

        <button
          class="ds_button"
          on:click={() => changePage("calculated-routes")}
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
      {:else if $subpage == "report"}
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
      {:else if $subpage == "disconnected"}
        <Disconnections />
      {:else if $subpage == "mesh-density"}
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
      {:else if $subpage == "streetspace"}
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
      {:else if $subpage == "calculated-routes"}
        <header class="ds_page-header">
          <h2 class="ds_page-header__title">Network impacts on demand</h2>
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

        <ODBreakdowns />
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
