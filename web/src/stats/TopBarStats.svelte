<script lang="ts">
  import { Loading } from "svelte-utils";
  import { Modal } from "../common";
  import { coherenceCombined, directness, safetyCombined } from "../stats";
  import {
    backend,
    lastUpdateSlowStats,
    mutationCounter,
    slowStats,
    stats,
  } from "../stores";
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
</script>

<Loading {loading} />

{#if $stats}
  <Modal bind:show={showSafety}>
    <Safety />
  </Modal>

  {#if $slowStats}
    <Modal bind:show={showDirectness}>
      <Directness bind:show={showDirectness} />
    </Modal>
  {/if}

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
          <progress value={safetyCombined($stats)[2]} max="100" />
        </a>
      </li>

      <li>
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a href="#" on:click|stopPropagation={openDirectness}>
          Directness
          {#if $slowStats && $lastUpdateSlowStats == $mutationCounter}
            <br />
            <progress value={directness($slowStats)[2]} max="100" />
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
          <progress value={coherenceCombined($stats)[2]} max="100" />
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
