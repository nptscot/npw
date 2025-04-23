<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { downloadGeneratedFile } from "svelte-utils";
  import { uncoveredPopulation } from "../layers/stores";
  import { backend, changeStage, devMode } from "../stores";
  import { subpage } from "./index";

  onMount(() => {
    $uncoveredPopulation = true;
  });
  onDestroy(() => {
    $uncoveredPopulation = false;
  });

  async function downloadZones() {
    let gj = await $backend!.getPopulationZones();
    downloadGeneratedFile("population_zones.geojson", JSON.stringify(gj));
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

<p>Zones with a red outline are not connected by the current network.</p>

<div>
  <button class="ds_button" on:click={() => changeStage("Secondary")}>
    Fix unconnected zones
  </button>
</div>

{#if $devMode}
  <div>
    <button class="ds_button ds_button--secondary" on:click={downloadZones}>
      Download zones
    </button>
  </div>
{/if}
