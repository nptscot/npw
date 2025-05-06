<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { BackLink } from "../common";
  import {
    backend,
    backgroundLayer,
    lastUpdateOD,
    mutationCounter,
    odStats,
  } from "../stores";
  import { subpage } from "./index";
  import ODStats from "./ODStats.svelte";

  onMount(async () => {
    $backgroundLayer = "calculated_rnet";

    if ($lastUpdateOD != $mutationCounter) {
      $odStats = await $backend!.recalculateODStats();
      $lastUpdateOD = $mutationCounter;
    }
  });
  onDestroy(() => {
    $backgroundLayer = "off";
  });
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network impacts on demand</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<p>
  This shows how journeys from census demand data would pick quiet routes using
  the proposed network.
</p>

{#if $odStats && $lastUpdateOD == $mutationCounter}
  <ODStats />
{:else}
  <p>Recalculating...</p>
{/if}
