<script lang="ts">
  import { Checkbox } from "../common";
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
  <Checkbox small bind:checked={$showUncovered}>
    Show all demand, even if covered
  </Checkbox>

  {#if $devMode}
    <Checkbox small bind:checked={$debugAllCyclingDemand}>
      Debug all demand
    </Checkbox>

    {#if $debugAllCyclingDemand}
      <label>
        Show demand above:
        <input type="number" bind:value={$debugCyclingDemandMin} />
      </label>
    {/if}
  {/if}
</LayerControls>
