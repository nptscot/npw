<script lang="ts">
  import { tierColors } from "./colors";
  import { tier } from "./stores";
  import { HelpButton } from "./common";
</script>

<div>
  <button
    style:background={$tier == "Primary" ? tierColors.Primary : "grey"}
    on:click={() => ($tier = "Primary")}
  >
    Primary routes
  </button>

  <button
    style:background={$tier == "Secondary" ? tierColors.Secondary : "grey"}
    on:click={() => ($tier = "Secondary")}
  >
    Secondary routes
  </button>

  <button
    style:background={$tier == "LocalAccess" ? tierColors.LocalAccess : "grey"}
    on:click={() => ($tier = "LocalAccess")}
  >
    Local access routes
  </button>

  <button
    style:background={$tier == "LongDistance"
      ? tierColors.LongDistance
      : "grey"}
    on:click={() => ($tier = "LongDistance")}
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
        spaces, neighbourhoods (especially deprived and densely populated ones),
        and above-minimum cycling flow routes on the base map.
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
