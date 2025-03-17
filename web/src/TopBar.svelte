<script lang="ts">
  import { tierLabels } from "./colors";
  import { Link } from "./common";
  import ManageFiles from "./common/ManageFiles.svelte";
  import { disableLayersPerStage, enableLayersPerStage } from "./layers/stores";
  import TopBarStats from "./stats/TopBarStats.svelte";
  import { currentStage, referenceRoadStyle } from "./stores";
  import type { Tier } from "./types";

  function changeStage(rawNewStage: string) {
    // Workaround TS
    let newStage = rawNewStage as Tier | "assessment";

    // Disable old layers
    for (let show of disableLayersPerStage[$currentStage]) {
      show.set(false);
    }
    if (
      $currentStage == "assessment" &&
      $referenceRoadStyle == "disconnections"
    ) {
      $referenceRoadStyle = "off";
    }

    $currentStage = newStage;

    // Show new layers
    for (let show of enableLayersPerStage[newStage]) {
      show.set(true);
    }

    if ($currentStage == "assessment") {
      $referenceRoadStyle = "disconnections";
    }
  }

  let stages = { ...tierLabels, assessment: "Assess" };
</script>

<div>
  <nav aria-label="breadcrumb">
    <ul>
      <li>NPW</li>
      <li><a href="index.html">Select area</a></li>

      <li><ManageFiles /></li>

      {#each Object.entries(stages) as [stage, label]}
        <li>
          {#if $currentStage == stage}
            <b>{label}</b>
          {:else}
            <Link on:click={() => changeStage(stage)}>{label}</Link>
          {/if}
        </li>
      {/each}
    </ul>
  </nav>

  <TopBarStats />
</div>

<style>
  div {
    display: flex;
    justify-content: space-between;
    padding: 4px;
  }

  nav {
    padding: 0 0.5rem;
  }

  nav ul {
    display: flex;
    flex-wrap: wrap;
    list-style: none;
    margin: 0;
    padding: 0;
    align-items: end;
  }

  nav li:not(:last-child)::after {
    display: inline-block;
    margin: 0 0.25rem;
    content: ">";
  }
</style>
