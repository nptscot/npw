<script lang="ts">
  import { Checkbox } from "../common";
  import LayerControls from "./LayerControls.svelte";
  import { mainRoadCoverage, showUncovered } from "./stores";
</script>

<LayerControls name="Main road coverage" bind:show={$mainRoadCoverage}>
  <Checkbox small bind:checked={$showUncovered}>
    Show all demand, even if covered
  </Checkbox>
</LayerControls>
