<script lang="ts">
  import { Loading } from "svelte-utils";
  import { Modal } from "../common";
  import {
    backend,
    changeStage,
    lastUpdateSlowStats,
    mutationCounter,
    slowStats,
    stats,
  } from "../stores";
  import SummarizeStats from "./SummarizeStats.svelte";

  let showStats = false;
  let loading = "";

  function gotoAssess() {
    changeStage("assessment");
    showStats = false;
  }

  // Returns something [0, 1]
  function percent(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }
    return (100 * x) / total;
  }

  async function openStats() {
    await recalculateDirectness();
    showStats = true;
  }

  async function recalculateDirectness() {
    if ($lastUpdateSlowStats == $mutationCounter) {
      return;
    }

    loading = "Recalculating directness";
    $slowStats = await $backend!.recalculateSlowStats();
    $lastUpdateSlowStats = $mutationCounter;
    loading = "";
  }

  // TODO Refactor with SummarizeStats
  function directnessScore(average_weighted_directness: number): number {
    if (average_weighted_directness > 1.5) {
      return 1;
    }
    if (average_weighted_directness > 1.4) {
      return 2;
    }
    if (average_weighted_directness > 1.3) {
      return 3;
    }
    if (average_weighted_directness > 1.2) {
      return 4;
    }
    return 5;
  }
</script>

<Loading {loading} />

{#if $stats}
  <Modal bind:show={showStats}>
    <button
      class="ds_button ds_button--secondary"
      on:click={gotoAssess}
      style="float: right"
    >
      See full assessment
    </button>
    <SummarizeStats />
  </Modal>

  <div class="progress-summary">
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|stopPropagation={openStats}>
      <ul>
        <li title="What percent of your network has high Level of Service?">
          Safety
          <br />
          <progress
            value={percent(
              $stats.total_high_los_length,
              $stats.total_network_length,
            )}
            max="100"
          />
        </li>

        <li title="Average weighted directness">
          Directness
          {#if $slowStats && $lastUpdateSlowStats == $mutationCounter}
            <br />
            <progress
              value={directnessScore($slowStats.average_weighted_directness)}
              max="5"
            />
          {:else}
            <a href="#" on:click|stopPropagation={recalculateDirectness}>
              <i class="fa-solid fa-calculator"></i>
            </a>
          {/if}
        </li>

        <li title="Percent of main roads covered by network">
          Coherence
          <br />
          <progress
            value={percent(
              $stats.covered_main_road_length,
              $stats.total_main_road_length,
            )}
            max="100"
          />
        </li>

        <!--<li title="Density of primary/secondary network within settlements">
        Coherence (density)
        <br />
        {#if $stats.density_network_in_settlements}
          {Math.round($stats.density_network_in_settlements)}m
        {:else}
          no routes yet
        {/if}
      </li>-->

        <li title="What percent of your network is on low gradient (&le; 3%)?">
          Comfort
          <br />
          <progress
            value={percent(
              $stats.total_low_gradient_length,
              $stats.total_network_length,
            )}
            max="100"
          />
        </li>

        <li title="What percent of your network is next to greenspace?">
          Attractiveness
          <br />
          <progress
            value={percent(
              $stats.total_attractive_length,
              $stats.total_network_length,
            )}
            max="100"
          />
        </li>
      </ul>
    </a>
  </div>
{/if}

<style>
  .progress-summary {
    padding-left: 20px;
    margin-left: auto;
  }

  .progress-summary a {
    text-decoration: none;
    color: #1a1a1a;
  }
  .progress-summary ul {
    height: 100%;
    list-style: none;
    display: flex;
    margin: 0;
    padding: 0;
  }
  .progress-summary ul li {
    list-style: none;
    font-size: 1rem;
    margin: 0;
    width: 100px;
    line-height: 12px;
    padding: 12px 6px 4px;
  }
  .progress-summary ul li progress {
    width: 100%;
  }
</style>
