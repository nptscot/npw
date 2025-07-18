<script lang="ts">
  import { onMount } from "svelte";
  import { Loading } from "svelte-utils";
  import { BackLink, Modal } from "../common";
  import SummarizeStats from "../stats/SummarizeStats.svelte";
  import {
    backend,
    lastUpdateOD,
    lastUpdateSlowStats,
    mutationCounter,
    odStats,
    slowStats,
  } from "../stores";
  import { subpage } from "./index";

  let show = true;
  $: if (!show) {
    $subpage = "overview";
  }

  let loading = "";

  onMount(async () => {
    if ($lastUpdateSlowStats != $mutationCounter) {
      loading = "Recalculating directness";
      $slowStats = await $backend!.recalculateSlowStats();
      $lastUpdateSlowStats = $mutationCounter;
      loading = "";
    }

    if ($lastUpdateOD != $mutationCounter) {
      loading = "Recalculating network impacts";
      $odStats = await $backend!.recalculateODStats();
      $lastUpdateOD = $mutationCounter;
      loading = "";
    }
  });

  let reportContent: HTMLDivElement | undefined = undefined;
  function printReport() {
    let newTab = window.open("", "_blank");
    if (!newTab || !reportContent) {
      return;
    }

    // Copy all the CSS/JS includes
    let headContents = document.head.innerHTML;

    let bodyContents = reportContent.outerHTML;
    let fullContents = `
			<!DOCTYPE html>
			<html lang="en">
			<head>
				${headContents}
			</head>
			<body>
        <header class="ds_page-header">
          <h2 class="ds_page-header__title">Network Planning Workspace final report</h2>
        </header>
				${bodyContents}
			</body>
			</html>
		`;
    newTab.document.write(fullContents);
  }
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network metrics</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<Loading {loading} />

{#if loading == ""}
  <Modal bind:show>
    {#if $slowStats && $lastUpdateSlowStats == $mutationCounter && $odStats && $lastUpdateOD == $mutationCounter}
      <div>
        <button class="ds_button" on:click={printReport}>Print report</button>
      </div>

      <div bind:this={reportContent} class="printable">
        <SummarizeStats />
      </div>
    {/if}
  </Modal>
{/if}

<style>
  @media screen {
    .printable {
      width: 760px;
      margin-left: auto;
      margin-right: auto;
    }
  }
</style>
