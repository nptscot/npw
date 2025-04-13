<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { downloadGeneratedFile } from "svelte-utils";
  import { uncoveredPopulation } from "../layers/stores";
  import { backend, devMode } from "../stores";
  import { subpage } from "./index";

  onMount(() => {
    $uncoveredPopulation = true;
  });
  onDestroy(() => {
    $uncoveredPopulation = false;
  });

  async function downloadDataZones() {
    let gj = await $backend!.getDataZones();
    downloadGeneratedFile("data_zones.geojson", JSON.stringify(gj));
  }
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Population coverage</h2>
</header>

<div>
  <button
    type="button"
    class="ds_link"
    on:click={() => ($subpage = "overview")}
  >
    <i class="fa-solid fa-chevron-left"></i>
    Back to network assessment
  </button>
</div>

<p>
  Zones with a red outline are not connected by the current network. To connect
  a zone, return to the Local Access tier and draw a route.
</p>

{#if $devMode}
  <button on:click={downloadDataZones}>Download data zones</button>
{/if}
