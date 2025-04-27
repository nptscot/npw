<script lang="ts">
  import {
    ArcElement,
    CategoryScale,
    Chart as ChartJS,
    Legend,
    Title,
    Tooltip,
  } from "chart.js";
  import { onDestroy, onMount } from "svelte";
  import { Pie } from "svelte-chartjs";
  import { QualitativeLegend } from "svelte-utils";
  import {
    infraTypeColorLegend,
    infraTypeColors,
    levelOfServiceColors,
    levelOfServiceLegend,
    tierColors,
  } from "../colors";
  import { BackLink } from "../common";
  import {
    backend,
    backgroundLayer,
    lastUpdateOD,
    mutationCounter,
    odStats,
  } from "../stores";
  import type { Tier } from "../types";
  import { subpage } from "./index";

  ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale);

  // Don't display the legend with ChartJS; it's too hard to control
  let options = {
    plugins: {
      legend: {
        display: false,
      },
    },
  };

  onMount(async () => {
    $backgroundLayer = "calculated_rnet";

    if ($lastUpdateOD != $mutationCounter) {
      $odStats = await $backend!.recalculateODStats();
      $lastUpdateOD = $mutationCounter;
    }
  });
  onDestroy(() => {
    $backgroundLayer = "off";
  });

  function castTier(x: string): Tier {
    return x as Tier;
  }
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network impacts on demand</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<p>
  This shows how journeys from census demand data would pick quiet routes using
  the network you have drawn.
</p>

{#if $odStats && $lastUpdateOD == $mutationCounter}
  <h4>Percent of demand by level of service</h4>
  <QualitativeLegend colors={levelOfServiceLegend} />
  <Pie
    data={{
      labels: Object.keys($odStats.od_percents_los),
      datasets: [
        {
          data: Object.values($odStats.od_percents_los).map((p) => p * 100),
          backgroundColor: Object.keys($odStats.od_percents_los).map(
            (key) => levelOfServiceColors[key],
          ),
        },
      ],
    }}
    {options}
  />

  <h4>Percent of demand by infrastructure type</h4>
  <QualitativeLegend
    colors={{ ...infraTypeColorLegend, "Not on the network": "grey" }}
  />
  <Pie
    data={{
      labels: Object.keys($odStats.od_percents_infra_type),
      datasets: [
        {
          data: Object.values($odStats.od_percents_infra_type).map(
            (p) => p * 100,
          ),
          backgroundColor: Object.keys($odStats.od_percents_infra_type).map(
            (key) => infraTypeColors[key] || "grey",
          ),
        },
      ],
    }}
    {options}
  />

  <h4>Percent of demand by tier</h4>
  <QualitativeLegend colors={{ ...tierColors, "Not on the network": "grey" }} />
  <Pie
    data={{
      labels: Object.keys($odStats.od_percents_tier),
      datasets: [
        {
          data: Object.values($odStats.od_percents_tier).map((p) => p * 100),
          backgroundColor: Object.keys($odStats.od_percents_tier).map(
            (key) => tierColors[castTier(key)] || "grey",
          ),
        },
      ],
    }}
    {options}
  />
{:else}
  <p>Recalculating...</p>
{/if}
