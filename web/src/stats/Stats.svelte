<script lang="ts">
  import { notNull } from "svelte-utils";
  import { backend, stats, mode, tier } from "../stores";
  import { tierColors } from "../colors";
  import { onMount } from "svelte";
  import {
    schools,
    gpHospitals,
    townCentres,
    deprivedPopulation,
    allPopulation,
    highRouteCoverage,
    settlementsCoverage,
  } from "../layers/stores";
  import Metric from "./Metric.svelte";

  async function recalc() {
    $stats = await $backend!.recalculateStats();
  }

  onMount(async () => {
    if ($stats == null) {
      await recalc();
    }
  });

  // Returns something [0, 1]
  function percent(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }
    return x / total;
  }
</script>

<button on:click={recalc}>Recalculate</button>

{#if $stats}
  <div style:border="2px solid {tierColors.Primary}">
    <Metric
      label="High cycling flow coverage"
      bind:showLayer={$highRouteCoverage}
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
        bind:showLayer={$highRouteCoverage}
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
        label="Low cycling flow coverage"
        bind:showLayer={$highRouteCoverage}
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
        label="Reachable settlements (TODO)"
        bind:showLayer={$settlementsCoverage}
        pct={0}
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

    <details>
      <summary>Percent of demand by infrastructure type</summary>
      <ul>
        {#each Object.entries($stats.od_percents_infra_type).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
          <li>{key}: {Math.round(pct * 100)}%</li>
        {/each}
      </ul>
    </details>

    <details>
      <summary>Percent of demand by level of service:</summary>
      <ul>
        {#each Object.entries($stats.od_percents_los).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
          <li>{key}: {Math.round(pct * 100)}%</li>
        {/each}
      </ul>
    </details>
  </div>
{/if}
