<script lang="ts">
  import { tierColors } from "./colors";
  import { HelpButton } from "./common";
  import { layersPerTier } from "./layers/stores";
  import { tier } from "./stores";
  import type { Tier } from "./types";

  function changeTier(newTier: Tier) {
    // Disable old layers
    for (let show of layersPerTier[$tier]) {
      show.set(false);
    }

    $tier = newTier;

    // Show new old layers
    for (let show of layersPerTier[newTier]) {
      show.set(true);
    }
  }
</script>

<div>
  <button
    style:background={$tier == "Primary" ? tierColors.Primary : "grey"}
    on:click={() => changeTier("Primary")}
  >
    Primary routes
  </button>

  <button
    style:background={$tier == "Secondary" ? tierColors.Secondary : "grey"}
    on:click={() => changeTier("Secondary")}
  >
    Secondary routes
  </button>

  <button
    style:background={$tier == "LocalAccess" ? tierColors.LocalAccess : "grey"}
    on:click={() => changeTier("LocalAccess")}
  >
    Local access routes
  </button>

  <button
    style:background={$tier == "LongDistance"
      ? tierColors.LongDistance
      : "grey"}
    on:click={() => changeTier("LongDistance")}
  >
    Long distance routes
  </button>

  <HelpButton>
    {#if $tier == "Primary"}
      <p>
        The primary route should be direct, coherent, meet high demand, and
        potentially connect settlements. Key primary routes will form Active
        Freeways. To draw the primary route, connect the high cycling flow
        routes on the base map.
      </p>
    {:else if $tier == "Secondary"}
      <p>
        To draw the secondary route, please connect town centres and cover
        medium cycling flow routes on the base map.
      </p>
    {:else if $tier == "LocalAccess"}
      <p>
        To draw the local access route, connect schools, GPs, hospitals, green
        spaces, and neighbourhoods (especially deprived and densely populated
        ones).
      </p>
    {:else if $tier == "LongDistance"}
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
    {/if}
  </HelpButton>
</div>

<style>
  div {
    display: flex;
  }

  button {
    flex: 1;
  }
</style>
