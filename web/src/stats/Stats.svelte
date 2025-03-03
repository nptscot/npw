<script lang="ts">
  import { Control } from "svelte-maplibre";
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
  <Control position="top-right">
    <div class="pico panel">
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

      Coherence (density of primary/secondary network in settlements):
      {#if $stats.density_network_in_settlements}
        <b>{Math.round($stats.density_network_in_settlements)}m</b>
      {:else}
        <b>no routes yet</b>
      {/if}
    </div>
  </Control>
{/if}

<style>
  .panel {
    background: white;
    max-width: 300px;
    max-height: 300px;
    overflow: auto;
  }
</style>
