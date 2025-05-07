<script lang="ts">
  import { Loading } from "svelte-utils";
  import { Modal } from "../common";
  import {
    backend,
    lastUpdateSlowStats,
    mutationCounter,
    slowStats,
    stats,
  } from "../stores";
  import type { Stats } from "../types";
  import Attractiveness from "./Attractiveness.svelte";
  import Coherence from "./Coherence.svelte";
  import Comfort from "./Comfort.svelte";
  import Directness from "./Directness.svelte";
  import Safety from "./Safety.svelte";

  let showSafety = false;
  let showDirectness = false;
  let showCoherence = false;
  let showComfort = false;
  let showAttractiveness = false;

  let loading = "";

  // Returns something [0, 1]
  function percent(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }
    return (100 * x) / total;
  }

  async function openDirectness() {
    await recalculateDirectness();
    showDirectness = true;
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

  // TODO Refactor with SummarizeStats
  function coherentDensityScore(s: Stats): number {
    if (
      !s.density_network_in_settlements ||
      s.density_network_in_settlements > 1000
    ) {
      return 1;
    }
    if (s.density_network_in_settlements > 500) {
      return 2;
    }
    if (s.density_network_in_settlements > 400) {
      return 3;
    }
    if (s.density_network_in_settlements > 250) {
      return 4;
    }
    return 5;
  }
</script>

<Loading {loading} />

{#if $stats}
  <Modal bind:show={showSafety}>
    <Safety />
  </Modal>

  <Modal bind:show={showDirectness}>
    <Directness />
  </Modal>

  <Modal bind:show={showCoherence}>
    <Coherence />
  </Modal>

  <Modal bind:show={showComfort}>
    <Comfort />
  </Modal>

  <Modal bind:show={showAttractiveness}>
    <Attractiveness />
  </Modal>

  <div class="progress-summary">
    <ul>
      <li>
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a href="#" on:click|stopPropagation={() => (showSafety = true)}>
          Safety
          <br />
          <progress
            value={percent(
              $stats.total_high_los_arterial_roads_length,
              $stats.total_arterial_road_length,
            )}
            max="100"
          />
        </a>
      </li>

      <li>
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a href="#" on:click|stopPropagation={openDirectness}>
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
        </a>
      </li>

      <li>
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a href="#" on:click|stopPropagation={() => (showCoherence = true)}>
          Coherence
          <br />
          <progress value={coherentDensityScore($stats)} max="5" />
        </a>
      </li>

      <li>
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a href="#" on:click|stopPropagation={() => (showComfort = true)}>
          Comfort
          <br />
          <progress
            value={percent(
              $stats.total_low_gradient_length,
              $stats.total_network_length,
            )}
            max="100"
          />
        </a>
      </li>

      <li>
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a
          href="#"
          on:click|stopPropagation={() => (showAttractiveness = true)}
        >
          Attractiveness
          <br />
          <progress
            value={percent(
              $stats.total_attractive_length,
              $stats.total_network_length,
            )}
            max="100"
          />
        </a>
      </li>
    </ul>
  </div>
{/if}

<style>
  a:hover {
    font-weight: bold;
  }

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
