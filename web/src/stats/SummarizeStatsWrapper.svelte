<script lang="ts">
  import { onMount } from "svelte";
  import { Loading } from "svelte-utils";
  import {
    backend,
    lastUpdateSlowStats,
    mutationCounter,
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
  });
</script>

<Loading {loading} />

{#if $slowStats && $lastUpdateSlowStats == $mutationCounter}
  <SummarizeStats />
{/if}
