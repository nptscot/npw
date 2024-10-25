<script lang="ts">
  import { backend, stats } from "./stores";
  import { onMount } from "svelte";

  async function recalc() {
    $stats = await $backend!.recalculateStats();
  }

  onMount(async () => {
    if ($stats == null) {
      await recalc();
    }
  });
</script>

<button on:click={recalc}>Recalculate</button>

{#if $stats}
  <p>
    Average weighted directness: {$stats.average_weighted_directness.toFixed(
      1,
    )}x
  </p>

  <p>Percent of demand by infrastructure type:</p>
  <ul>
    {#each Object.entries($stats.od_percents) as [key, percent]}
      <li>{key}: {(100 * percent).toFixed(1)}%</li>
    {/each}
  </ul>
{/if}
