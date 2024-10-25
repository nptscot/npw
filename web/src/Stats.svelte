<script lang="ts">
  import { backend, odZones, odPairs, stats } from "./stores";
  import { onMount } from "svelte";

  async function recalc() {
    $stats = await $backend!.recalculateStats($odZones, $odPairs);
  }

  onMount(async () => {
    if ($stats == null) {
      await recalc();
    }
  });
</script>

<button on:click={recalc}>Recalculate</button>

{#if $stats}
  <p>Percent of demand by infrastructure type:</p>
  <ul>
    {#each Object.entries($stats.od_percents) as [key, percent]}
      <li>{key}: {(100 * percent).toFixed(1)}%</li>
    {/each}
  </ul>
{/if}
