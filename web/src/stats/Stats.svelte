<script lang="ts">
  import { Control } from "svelte-maplibre";
  import { Loading, notNull } from "svelte-utils";
  import { tierColors } from "../colors";
  import { backend, mode, mutationCounter, odStats, stats } from "../stores";
  import FinalReport from "./FinalReport.svelte";
  import Metric from "./Metric.svelte";
  import ODBreakdowns from "./ODBreakdowns.svelte";

  // Start less than $mutationCounter
  let lastUpdateFast = 0;
  let lastUpdateOD = 0;
  let loading = "";

  async function recalcFast() {
    if ($backend && lastUpdateFast != $mutationCounter) {
      console.time("Recalculate fast stats");
      $stats = await $backend.recalculateStats();
      console.timeEnd("Recalculate fast stats");
      lastUpdateFast = $mutationCounter;
    }
  }
  $: if ($mutationCounter > 0) {
    recalcFast();
  }

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
  <Control position="top-right">
    <div class="pico panel">
      <details open>
        <summary>Stats</summary>

        <div style:padding="4px" style:border="2px solid {tierColors.Primary}">
          <Metric
            label="High cycling flow coverage"
            pct={percent(
              $stats.covered_flow_quintile_sums[0],
              $stats.total_flow_quintile_sums[0],
            )}
          />
        </div>

        <div
          style:padding="4px"
          style:margin-top="4px"
          style:border="2px solid {tierColors.Secondary}"
        >
          <Metric
            label="Medium cycling flow coverage"
            pct={percent(
              $stats.covered_flow_quintile_sums[1],
              $stats.total_flow_quintile_sums[1],
            )}
          />

          <Metric
            label="Town centres"
            pct={$stats.percent_reachable_town_centres}
          />
        </div>

        <div
          style:padding="4px"
          style:margin-top="4px"
          style:border="2px solid {tierColors.LocalAccess}"
        >
          <Metric label="Schools" pct={$stats.percent_reachable_schools} />

          <Metric
            label="GPs and hospitals"
            pct={$stats.percent_reachable_gp_hospitals}
          />

          <Metric
            label="Greenspaces"
            pct={$stats.percent_reachable_greenspaces}
          />

          <Metric
            label="Deprived population coverage"
            pct={$stats.percent_reachable_imd_population}
          />

          <Metric
            label="Population coverage"
            pct={$stats.percent_reachable_population}
          />
        </div>

        <div
          style:padding="4px"
          style:margin-top="4px"
          style:border="2px solid {tierColors.LongDistance}"
        >
          <Metric
            label="Settlements"
            pct={$stats.percent_reachable_settlements}
          />
        </div>

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
                    kind: "evaluate-route",
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
            Initial stats calculation disabled for faster start. Press the
            button above.
          </p>
        {/if}
      </details>
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
