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
  import { backend, mode, mutationCounter, stats, tier } from "../stores";
  import Metric from "./Metric.svelte";
  import ODBreakdowns from "./ODBreakdowns.svelte";

  // Start less than $mutationCounter
  let lastUpdate = 0;
  let loading = "";

  async function recalc() {
    loading = "Recalculating stats";
    $stats = await $backend!.recalculateStats();
    loading = "";
    lastUpdate = $mutationCounter;
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

<button on:click={recalc} disabled={$mutationCounter == lastUpdate}>
  Recalculate
</button>

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

  <div style:margin-top="4px" style:border="2px solid black">
    <p>
      <!-- svelte-ignore a11y-invalid-attribute -->
      <a
        href="#"
        on:click|preventDefault={() =>
          ($mode = {
            kind: "evaluate-route",
            prevMode: { kind: "main" },
            browse: notNull($stats).worst_directness_routes,
          })}
      >
        Average weighted directness
      </a>
      : {$stats.average_weighted_directness.toFixed(1)}x
    </p>

    <ODBreakdowns od={$stats} />
  </div>
{:else}
  <p>
    Initial stats calculation disabled for faster start. Press the button above.
  </p>
{/if}
