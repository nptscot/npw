<script lang="ts">
  import { devMode } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import {
    debugAllCyclingDemand,
    debugCyclingDemandMin,
    showUncovered,
  } from "./stores";

  export let show: boolean;
  export let label: string;
</script>

<LayerControls name={label + " cycling demand"} bind:show>
  <label>
    <input type="checkbox" bind:checked={$showUncovered} role="switch" />
    Show all demand, even if covered
  </label>

  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={$debugAllCyclingDemand} />
      Debug all demand
    </label>

    {#if $debugAllCyclingDemand}
      <label>
        Show demand above:
        <input type="number" bind:value={$debugCyclingDemandMin} />
      </label>
    {/if}
  {/if}
</LayerControls>
