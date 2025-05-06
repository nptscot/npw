<script lang="ts">
  import { onMount } from "svelte";
  import { Loading } from "svelte-utils";
  import {
    backend,
    lastUpdateOD,
    lastUpdateSlowStats,
    mutationCounter,
    odStats,
    slowStats,
  } from "../stores";
  import SummarizeStats from "./SummarizeStats.svelte";

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
</script>

<Loading {loading} />

{#if $slowStats && $lastUpdateSlowStats == $mutationCounter && $odStats && $lastUpdateOD == $mutationCounter}
  <SummarizeStats />
{/if}
