<script lang="ts">
  import { onMount } from "svelte";
  import { Loading } from "svelte-utils";
  import { backend, lastUpdateOD, mutationCounter, odStats } from "../stores";
  import SummarizeStats from "./SummarizeStats.svelte";

  let loading = "";

  onMount(async () => {
    if ($lastUpdateOD != $mutationCounter) {
      loading = "Recalculating directness";
      $odStats = await $backend!.recalculateODStats();
      $lastUpdateOD = $mutationCounter;
      loading = "";
    }
  });
</script>

<Loading {loading} />

{#if $odStats && $lastUpdateOD == $mutationCounter}
  <SummarizeStats />
{/if}
