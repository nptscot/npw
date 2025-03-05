<script lang="ts">
  import { Loading, notNull } from "svelte-utils";
  import {
    backend,
    currentStage,
    mode,
    mutationCounter,
    odStats,
    stats,
  } from "../stores";
  import FinalReport from "./FinalReport.svelte";
  import Metric from "./Metric.svelte";
  import ODBreakdowns from "./ODBreakdowns.svelte";

  let loading = "";
  // TODO Will this state get lost as the user navigates around? Seemingly no,
  // because its only used in a stage that doesn't have much editing.
  let lastUpdateOD = 0;

  async function recalcOD() {
    loading = "Recalculating OD stats";
    $odStats = await $backend!.recalculateODStats();
    loading = "";
    lastUpdateOD = $mutationCounter;
  }

  // Returns something [0, 1]
  function percent(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }
    return x / total;
  }
</script>

<Loading {loading} />

{#if $stats}
  <h4>Assess this tier</h4>

  {#if $currentStage == "Primary"}
    <Metric
      label="High cycling demand coverage"
      pct={percent(
        $stats.covered_demand_quintile_sums[0],
        $stats.total_demand_quintile_sums[0],
      )}
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
      pct={percent(
        $stats.covered_demand_quintile_sums[1],
        $stats.total_demand_quintile_sums[1],
      )}
    />

    <Metric label="Town centres" pct={$stats.percent_reachable_town_centres} />
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
  {:else if $currentStage == "assessment"}
    <FinalReport />

    <button
      class="outline"
      on:click={recalcOD}
      disabled={$mutationCounter == lastUpdateOD}
    >
      Recalculate
    </button>

    {#if $odStats}
      <div style:margin-top="4px" style:border="2px solid black">
        <p>
          <!-- svelte-ignore a11y-invalid-attribute -->
          <a
            href="#"
            on:click|preventDefault={() =>
              ($mode = {
                kind: "evaluate-journey",
                prevMode: { kind: "main" },
                browse: notNull($odStats).worst_directness_routes,
              })}
          >
            Average weighted directness
          </a>
          : {$odStats.average_weighted_directness.toFixed(1)}x
        </p>

        <ODBreakdowns od={$odStats} />
      </div>
    {:else}
      <p>
        Initial stats calculation disabled for faster start. Press the button
        above.
      </p>
    {/if}
  {/if}
{/if}
