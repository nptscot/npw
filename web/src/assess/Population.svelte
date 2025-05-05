<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { downloadGeneratedFile } from "svelte-utils";
  import { BackLink } from "../common";
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
    let gj = await $backend!.getDataZones();
    downloadGeneratedFile("population_zones.geojson", JSON.stringify(gj));
  }
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Neighbourhood coverage</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<p>
  Neighbourhoods with a red outline are not connected by the current network.
</p>

<div>
  <button class="ds_button" on:click={() => changeStage("Secondary")}>
    Fix unconnected neighbourhoods
  </button>
</div>

{#if $devMode}
  <div>
    <button class="ds_button ds_button--secondary" on:click={downloadZones}>
      Download neighbourhoods
    </button>
  </div>
{/if}
