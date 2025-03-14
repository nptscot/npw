<script lang="ts">
  import { tierLabels } from "./colors";
  import { HelpButton, Link } from "./common";
  import ManageFiles from "./common/ManageFiles.svelte";
  import { disableLayersPerStage, enableLayersPerStage } from "./layers/stores";
  import TopBarStats from "./stats/TopBarStats.svelte";
  import { currentStage, devMode, referenceRoadStyle } from "./stores";
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

  let stages = { ...tierLabels, assessment: "Network assessment" };
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

  <HelpButton>
    {#if $currentStage == "Primary"}
      <p>
        The primary route should be direct, coherent, meet high demand, and
        potentially connect settlements. Key primary routes will form Active
        Freeways. To draw the primary route, connect the high cycling demand
        routes on the base map.
      </p>
    {:else if $currentStage == "Secondary"}
      <p>
        To draw the secondary route, please connect town centres and cover
        medium cycling demand routes on the base map.
      </p>
    {:else if $currentStage == "LocalAccess"}
      <p>
        To draw the local access route, connect schools, GPs, hospitals, green
        spaces, and neighbourhoods (especially deprived and densely populated
        ones).
      </p>
    {:else if $currentStage == "LongDistance"}
      <ul>
        <li>
          Long distance routes connect EDJ reachable settlements out with main
          urban areas.
        </li>
        <li>
          Settlements should be connected by high demand routes forming a direct
          connection in most cases.
        </li>
        <li>
          Long distance routes should connect directly to primary routes within
          each settlement.
        </li>
        <li>Consider SIMD/transport poverty</li>
        <li>
          In limited circumstances, settlement can be connected by less
          direct/more scenic routes (NCN)
        </li>
      </ul>
    {:else if $currentStage == "assessment"}
      <p>
        Having designed your network, you can now assess its performance and fix
        any problems.
      </p>
    {/if}

    <label>
      <input type="checkbox" bind:checked={$devMode} />
      Dev mode
    </label>
  </HelpButton>
</div>

<TopBarStats />

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
