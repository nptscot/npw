<script lang="ts">
  import { stats } from "../stores";
  import TerseMetric from "./TerseMetric.svelte";

  // Returns something [0, 1]
  function percent(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }
    return x / total;
  }
</script>

{#if $stats}
  <div class="progress" style:display="flex" style:gap="3em">
    <TerseMetric
      label="Safety"
      pct={percent($stats.total_high_los_length, $stats.total_network_length)}
      tooltip="What percent of your network has high Level of Service?"
    />

    <TerseMetric
      label="Comfort"
      pct={percent(
        $stats.total_low_gradient_length,
        $stats.total_network_length,
      )}
      tooltip="What percent of your network is on low gradient (&le; 3%)?"
    />

    <TerseMetric
      label="Coherence (main roads)"
      pct={percent(
        $stats.covered_main_road_length,
        $stats.total_main_road_length,
      )}
    />

    <div
      style="display: flex; flex-direction: column"
      title="Density of primary/secondary network within settlements"
    >
      <span>Coherence (density)</span>
      {#if $stats.density_network_in_settlements}
        <b>{Math.round($stats.density_network_in_settlements)}m</b>
      {:else}
        <b>no routes yet</b>
      {/if}
    </div>
  </div>
{/if}
