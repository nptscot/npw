<script lang="ts">
  import { Loading, notNull } from "svelte-utils";
  import { tierColors } from "../colors";
  import {
    allPopulation,
    cyclingFlow1,
    cyclingFlow2,
    cyclingFlow3,
    deprivedPopulation,
    gpHospitals,
    greenspaces,
    schools,
    settlements,
    townCentres,
  } from "../layers/stores";
  import {
    backend,
    mode,
    mutationCounter,
    odStats,
    stats,
    tier,
  } from "../stores";
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
  <div style:border="2px solid {tierColors.Primary}">
    <Metric
      label="High cycling flow coverage"
      bind:showLayer={$cyclingFlow1}
      pct={percent(
        $stats.covered_flow_quintile_sums[0],
        $stats.total_flow_quintile_sums[0],
      )}
    />
  </div>

  {#if $tier == "Secondary" || $tier == "LocalAccess" || $tier == "LongDistance"}
    <div style:margin-top="4px" style:border="2px solid {tierColors.Secondary}">
      <Metric
        label="Medium cycling flow coverage"
        bind:showLayer={$cyclingFlow2}
        pct={percent(
          $stats.covered_flow_quintile_sums[1],
          $stats.total_flow_quintile_sums[1],
        )}
      />

      <Metric
        label="Town centres"
        bind:showLayer={$townCentres}
        pct={$stats.percent_reachable_town_centres}
      />
    </div>
  {/if}

  {#if $tier == "LocalAccess" || $tier == "LongDistance"}
    <div
      style:margin-top="4px"
      style:border="2px solid {tierColors.LocalAccess}"
    >
      <Metric
        label="Above-minimum cycling flow coverage"
        bind:showLayer={$cyclingFlow3}
        pct={percent(
          $stats.covered_flow_quintile_sums[2],
          $stats.total_flow_quintile_sums[2],
        )}
      />

      <Metric
        label="Schools"
        bind:showLayer={$schools}
        pct={$stats.percent_reachable_schools}
      />

      <Metric
        label="GPs and hospitals"
        bind:showLayer={$gpHospitals}
        pct={$stats.percent_reachable_gp_hospitals}
      />

      <Metric
        label="Reachable greenspaces"
        bind:showLayer={$greenspaces}
        pct={$stats.percent_reachable_greenspaces}
      />

      <Metric
        label="Deprived population coverage"
        bind:showLayer={$deprivedPopulation}
        pct={$stats.percent_reachable_imd_population}
      />

      <Metric
        label="Population coverage"
        bind:showLayer={$allPopulation}
        pct={$stats.percent_reachable_population}
      />
    </div>
  {/if}

  {#if $tier == "LongDistance"}
    <div
      style:margin-top="4px"
      style:border="2px solid {tierColors.LongDistance}"
    >
      <Metric
        label="Reachable settlements"
        bind:showLayer={$settlements}
        pct={$stats.percent_reachable_settlements}
      />
    </div>
  {/if}
{/if}

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
    Initial stats calculation disabled for faster start. Press the button above.
  </p>
{/if}
