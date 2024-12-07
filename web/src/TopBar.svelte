<script lang="ts">
  import { tierColors } from "./colors";
  import { tier } from "./stores";
  import { HelpButton } from "./common";
</script>

<div>
  <button
    style:background={$tier == "primary" ? tierColors.primaryRoutes : "grey"}
    on:click={() => ($tier = "primary")}
  >
    Primary routes
  </button>

  <button
    style:background={$tier == "secondary"
      ? tierColors.secondaryRoutes
      : "grey"}
    on:click={() => ($tier = "secondary")}
  >
    Secondary routes
  </button>

  <button
    style:background={$tier == "local access"
      ? tierColors.localAccessRoutes
      : "grey"}
    on:click={() => ($tier = "local access")}
  >
    Local access routes
  </button>

  <button
    style:background={$tier == "long distance"
      ? tierColors.longDistanceRoutes
      : "grey"}
    on:click={() => ($tier = "long distance")}
  >
    Long distance routes
  </button>

  <HelpButton>
    {#if $tier == "primary"}
      <p>
        The primary route should be direct, coherent, meet high demand, and
        potentially connect settlements. Key primary routes will form Active
        Freeways. To draw the primary route, connect the high cycling flow
        routes on the base map.
      </p>
    {:else if $tier == "secondary"}
      <p>
        To draw the secondary route, please connect town centres and cover
        medium cycling flow routes on the base map.
      </p>
    {:else if $tier == "local access"}
      <p>
        To draw the local access route, connect schools, GPs, hospitals, green
        spaces, neighbourhoods (especially deprived and densely populated ones),
        and above-minimum cycling flow routes on the base map.
      </p>
    {:else if $tier == "long distance"}
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
