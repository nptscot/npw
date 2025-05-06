<script lang="ts">
  import { downloadGeneratedFile } from "svelte-utils";
  import PrintableReport from "./assess/PrintableReport.svelte";
  import { BackLink } from "./common";
  import { SplitComponent } from "./common/layout";
  import {
    backend,
    boundaryName,
    currentFilename,
    lastUpdateOD,
    lastUpdateSlowStats,
    mode,
    mutationCounter,
    odStats,
    slowStats,
  } from "./stores";

  let reportContent: HTMLDivElement | undefined = undefined;

  function makeReport() {
    let newTab = window.open("", "_blank");
    if (!newTab || !reportContent) {
      return;
    }

    // Copy all the CSS/JS includes
    let headContents = document.head.innerHTML;

    // Temporarily render the report, to grab its DOM
    reportContent.style.display = "block";
    let bodyContents = reportContent.innerHTML;
    let fullContents = `
			<!DOCTYPE html>
			<html lang="en">
			<head>
				${headContents}
			</head>
			<body>
				${bodyContents}
			</body>
			</html>
		`;
    newTab.document.write(fullContents);

    reportContent.style.display = "none";
  }

  async function exportFile() {
    let file = `npw_${$boundaryName}_${$currentFilename}.geojson`;
    downloadGeneratedFile(file, JSON.stringify(await $backend!.getAllRoutes()));
  }
</script>

<SplitComponent>
  <svelte:fragment slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Export project</h2>
      </header>

      <BackLink on:click={() => ($mode = { kind: "overview" })}>
        Back to project overview
      </BackLink>

      <p>Export the project to share with stakeholders.</p>

      <div>
        {#if $slowStats && $lastUpdateSlowStats == $mutationCounter && $odStats && $lastUpdateOD == $mutationCounter}
          <button class="ds_button" on:click={makeReport}>Print report</button>
        {:else}
          <button class="ds_button" disabled>Print report (loading...)</button>
        {/if}
      </div>

      <div>
        <button class="ds_button" on:click={exportFile}>
          Export project file to share
        </button>
      </div>
    </div>
  </svelte:fragment>
</SplitComponent>

<!-- Render an invisible component to steal its HTML -->
<div bind:this={reportContent} style:display="none">
  <PrintableReport />
</div>

<style>
  .main-controls {
    height: 100%;
    overflow-y: auto;
    padding: 20px;
  }
</style>
