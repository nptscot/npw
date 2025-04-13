<script lang="ts">
  import { controlsContents, mapContents } from "./index.js";

  // The kinda weird hack is that this must itself be nested underneath the
  // MapLibre bit, so it has context. The sidebars are the "remote" parts.
</script>

<div bind:this={$controlsContents} style:height="100%">
  <slot name="controls" />
</div>
<div bind:this={$mapContents}>
  <slot name="map" />
</div>
