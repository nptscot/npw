<script lang="ts">
  import { stats } from "../stores";
  import { percent } from "../utils";
  import LayerControls from "./LayerControls.svelte";

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
