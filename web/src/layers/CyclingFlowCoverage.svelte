<script lang="ts">
  import { percent } from "../common";
  import { devMode, stats } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import { debugAllCyclingFlow, debugCyclingFlowMin } from "./stores";

  export let quintile: number;
  export let show: boolean;
  export let label: string;
</script>

<LayerControls name={label + " cycling flow"} bind:show>
  {#if $stats}
    <span class="legend" />

    <p>
      {percent(
        $stats.covered_flow_quintile_sums[quintile - 1],
        $stats.total_flow_quintile_sums[quintile - 1],
      )} of quintile {quintile} flows covered
    </p>
  {/if}

  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={$debugAllCyclingFlow} />
      Debug all flows
    </label>

    {#if $debugAllCyclingFlow}
      <label>
        Show flows above:
        <input type="number" bind:value={$debugCyclingFlowMin} />
      </label>
    {/if}
  {/if}
</LayerControls>

<style>
  .legend {
    float: left;
    height: 16px;
    width: 30px;
    margin-right: 5px;
    background-color: grey;
  }
</style>
