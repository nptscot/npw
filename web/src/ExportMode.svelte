<script lang="ts">
  import { downloadGeneratedFile } from "svelte-utils";
  import PrintableReport from "./assess/PrintableReport.svelte";
  import { SplitComponent } from "./common/layout";
  import { backend, boundaryName, currentFilename, mode } from "./stores";

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

  async function exportFile() {
    let file = `npw_${$boundaryName}_${$currentFilename}.geojson`;
    downloadGeneratedFile(file, JSON.stringify(await $backend!.getAllRoutes()));
  }
</script>

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Export project</h2>
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

      <p>Export the project to share with stakeholders.</p>

      <div>
        <button class="ds_button" on:click={makeReport}>Print report</button>
      </div>

      <div>
        <button class="ds_button" on:click={exportFile}>
          Export project file to share
        </button>
      </div>
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
