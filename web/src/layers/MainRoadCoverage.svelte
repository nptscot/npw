<script lang="ts">
  import LayerControls from "./LayerControls.svelte";
  import { mainRoadCoverage } from "./stores";
</script>

<LayerControls name="Main road coverage" bind:show={$mainRoadCoverage} empty />
