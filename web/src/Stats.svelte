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

  function percent(x: number): string {
    return `${(100 * x).toFixed(1)}%`;
  }
</script>

<button on:click={recalc}>Recalculate</button>

{#if $stats}
  <p>
    Average weighted directness: {$stats.average_weighted_directness.toFixed(
      1,
    )}x
  </p>

  <p>
    Schools: <b>{percent($stats.percent_reachable_schools)}</b>
  </p>

  <details>
    <summary>Percent of demand by infrastructure type</summary>
    <ul>
      {#each Object.entries($stats.od_percents_infra_type).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
        <li>{key}: {percent(pct)}</li>
      {/each}
    </ul>
  </details>

  <details>
    <summary>Percent of demand by level of service:</summary>
    <ul>
      {#each Object.entries($stats.od_percents_los).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
        <li>{key}: {percent(pct)}</li>
      {/each}
    </ul>
  </details>
{/if}
