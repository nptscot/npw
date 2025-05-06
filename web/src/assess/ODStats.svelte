<script lang="ts">
  import {
    ArcElement,
    CategoryScale,
    Chart as ChartJS,
    Legend,
    Title,
    Tooltip,
  } from "chart.js";
  import { Pie } from "svelte-chartjs";
  import { QualitativeLegend } from "svelte-utils";
  import {
    infraTypeColorLegend,
    infraTypeColors,
    levelOfServiceColors,
    levelOfServiceLegend,
    tierColors,
  } from "../colors";
  import { odStats } from "../stores";
  import type { Tier } from "../types";

  ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale);

  // Don't display the legend with ChartJS; it's too hard to control
  let options = {
    plugins: {
      legend: {
        display: false,
      },
    },
  };

  function castTier(x: string): Tier {
    return x as Tier;
  }
</script>

{#if $odStats}
  <h4>Percent of demand by level of service</h4>
  <QualitativeLegend labelColors={levelOfServiceLegend} />
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
    labelColors={{
      ...infraTypeColorLegend,
      "Not part of designated network": "grey",
    }}
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
  <QualitativeLegend
    labelColors={{ ...tierColors, "Not part of designated network": "grey" }}
  />
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
{/if}
