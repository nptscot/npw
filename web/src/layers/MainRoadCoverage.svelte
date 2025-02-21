<script lang="ts">
  import { percent } from "../common";
  import { stats } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import { mainRoadCoverage } from "./stores";
</script>

<LayerControls name="Main road coverage" bind:show={$mainRoadCoverage}>
  {#if $stats}
    <span class="legend" />

    <p>
      {percent($stats.covered_main_road_length, $stats.total_main_road_length)} of
      main roads covered
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
