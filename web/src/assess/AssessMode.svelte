<script lang="ts">
  import { SplitComponent } from "../common/layout";
  import { exitCurrentStage, mode } from "../stores";
  import Disconnections from "./Disconnections.svelte";
  import FinalReport from "./FinalReport.svelte";
  import { changePage, subpage } from "./index";
  import MeshDensity from "./MeshDensity.svelte";
  import ODBreakdowns from "./ODBreakdowns.svelte";
  import PrintableReport from "./PrintableReport.svelte";
  import Streetspace from "./Streetspace.svelte";

  function gotoOverview() {
    exitCurrentStage();
    $mode = { kind: "overview" };
  }

  let reportContent: HTMLDivElement | undefined = undefined;

  function makeReport() {
    let newTab = window.open("", "_blank");
    if (!newTab || !reportContent) {
      return;
    }

    reportContent.style.display = "block";
    newTab.document.write(reportContent.outerHTML);
    reportContent.style.display = "none";
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

        <div>
          <button class="ds_button" on:click={() => changePage("report")}>
            Show network metrics
          </button>
        </div>

        <div>
          <button class="ds_button" on:click={makeReport}>Print report</button>
        </div>

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
        <FinalReport />
      {:else if $subpage == "disconnected"}
        <Disconnections />
      {:else if $subpage == "mesh-density"}
        <MeshDensity />
      {:else if $subpage == "streetspace"}
        <Streetspace />
      {:else if $subpage == "calculated-routes"}
        <ODBreakdowns />
      {/if}
    </div>
  </div>
</SplitComponent>

<!-- Render an invisible component to steal its HTML -->
<div bind:this={reportContent} style:display="none">
  <PrintableReport />
</div>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
