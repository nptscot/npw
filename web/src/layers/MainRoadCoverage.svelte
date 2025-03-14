<script lang="ts">
  import LayerControls from "./LayerControls.svelte";
  import { mainRoadCoverage, showUncovered } from "./stores";
</script>

<LayerControls name="Main road coverage" bind:show={$mainRoadCoverage}>
  <label>
    <input type="checkbox" bind:checked={$showUncovered} role="switch" />
    Show all demand, even if covered
  </label>
</LayerControls>
