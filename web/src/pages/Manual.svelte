<script lang="ts">
  import "@scottish-government/design-system/dist/css/design-system.css";
  import "@scottish-government/design-system/dist/scripts/design-system.js";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import { onMount } from "svelte";

  onMount(async () => {
    // @ts-expect-error This really exists for the SG design system, but TS doesn't know about it
    window.DS.initAll();
  });
</script>

<h1>Network Planning Workspace User Manual</h1>

<div>
  <a href="./">
    <i class="fa-solid fa-chevron-left"></i>
    Back to NPW
  </a>
</div>

<p>Coming soon</p>
