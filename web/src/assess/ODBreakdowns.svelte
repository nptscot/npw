<script lang="ts">
  import {
    ArcElement,
    CategoryScale,
    Chart as ChartJS,
    Legend,
    Title,
    Tooltip,
  } from "chart.js";
  import { onMount } from "svelte";
  import { Pie } from "svelte-chartjs";
  import { infraTypeColors, levelOfServiceColors, tierColors } from "../colors";
  import { backend, lastUpdateOD, mutationCounter, odStats } from "../stores";
  import { changePage } from "./index";

  ChartJS.register(Title, Tooltip, Legend, ArcElement, CategoryScale);

  onMount(async () => {
    if ($lastUpdateOD != $mutationCounter) {
      $odStats = await $backend!.recalculateODStats();
      $lastUpdateOD = $mutationCounter;
    }
  });
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network impacts on demand</h2>
</header>

<div>
  <button type="button" class="ds_link" on:click={() => changePage("overview")}>
    <i class="fa-solid fa-chevron-left"></i>
    Back to network assessment
  </button>
</div>

{#if $odStats && $lastUpdateOD == $mutationCounter}
  <h4>Percent of demand by level of service</h4>
  <div style:height="300px">
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
      options={{ responsive: true }}
    />
  </div>

  <h4>Percent of demand by infrastructure type</h4>
  <div style:height="300px">
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
      options={{ responsive: true }}
    />
  </div>

  <h4>Percent of demand by tier</h4>
  <div style:height="300px">
    <Pie
      data={{
        labels: Object.keys($odStats.od_percents_tier),
        datasets: [
          {
            data: Object.values($odStats.od_percents_tier).map((p) => p * 100),
            backgroundColor: Object.keys($odStats.od_percents_tier).map(
              (key) => tierColors[key] || "grey",
            ),
          },
        ],
      }}
      options={{ responsive: true }}
    />
  </div>
{:else}
  <p>Recalculating...</p>
{/if}
