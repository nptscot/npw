<script lang="ts">
  import { stats } from "../stores";
  import Metric from "./Metric.svelte";

  // Returns something [0, 1]
  function percent(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }
    return x / total;
  }
</script>

{#if $stats}
  <div>
    <Metric
      label="Safety (high LoS)"
      pct={percent($stats.total_high_los_length, $stats.total_network_length)}
    />

    <Metric
      label="Comfort (low gradient)"
      pct={percent(
        $stats.total_low_gradient_length,
        $stats.total_network_length,
      )}
    />

    <Metric
      label="Coherence (main road coverage)"
      pct={percent(
        $stats.covered_main_road_length,
        $stats.total_main_road_length,
      )}
    />

    <div style="display: flex; flex-direction: column">
      <span>
        Coherence (density of primary/secondary network in settlements):
      </span>
      <span>
        {#if $stats.density_network_in_settlements}
          <b>{Math.round($stats.density_network_in_settlements)}m</b>
        {:else}
          <b>no routes yet</b>
        {/if}
      </span>
    </div>
  </div>
{/if}

<style>
  div {
    display: flex;
    justify-content: space-between;
  }
</style>
