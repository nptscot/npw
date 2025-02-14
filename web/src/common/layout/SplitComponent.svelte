<script lang="ts">
  import { leftSidebarContents, mapContents } from "./index.js";

  // The kinda weird hack is that this must itself be nested underneath the
  // MapLibre bit, so it has context. The sidebars are the "remote" parts.
</script>

<div bind:this={$leftSidebarContents}>
  <slot name="left" />
</div>
<div bind:this={$mapContents}>
  <slot name="map" />
</div>
