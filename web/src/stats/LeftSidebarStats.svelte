<script lang="ts">
  import { currentStage, mode, stats } from "../stores";
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
  <div class="assess">
    <div style="display: flex; justify-content: space-between;">
      <h3>Assess this tier</h3>
      <!-- svelte-ignore a11y-invalid-attribute -->
      <a
        href="#"
        on:click|preventDefault={() =>
          ($mode = {
            kind: "evaluate-journey",
            browse: [],
          })}
      >
        Evaluate a journey
      </a>
    </div>

    {#if $currentStage == "Primary"}
      <Metric
        label="High cycling demand coverage"
        pct={percent($stats.covered_high_demand, $stats.total_high_demand)}
      />

      <Metric
        label="Main road coverage"
        pct={percent(
          $stats.covered_main_road_length,
          $stats.total_main_road_length,
        )}
      />
    {:else if $currentStage == "Secondary"}
      <Metric
        label="Medium cycling demand coverage"
        pct={percent($stats.covered_medium_demand, $stats.total_medium_demand)}
      />

      <Metric
        label="Town centres"
        pct={$stats.percent_reachable_town_centres}
      />
    {:else if $currentStage == "LocalAccess"}
      <Metric label="Schools" pct={$stats.percent_reachable_schools} />

      <Metric
        label="GPs and hospitals"
        pct={$stats.percent_reachable_gp_hospitals}
      />

      <Metric label="Greenspaces" pct={$stats.percent_reachable_greenspaces} />

      <Metric
        label="Deprived population coverage"
        pct={$stats.percent_reachable_imd_population}
      />

      <Metric
        label="Population coverage"
        pct={$stats.percent_reachable_population}
      />
    {:else if $currentStage == "LongDistance"}
      <Metric label="Settlements" pct={$stats.percent_reachable_settlements} />
    {/if}
  </div>
{/if}

<style>
  .assess {
    margin-top: auto;
    height: auto;
    width: 100%;
    padding: 0 20px;
    border-top: 1px solid #ccc;
    padding: 12px 20px 8px;
  }
</style>
