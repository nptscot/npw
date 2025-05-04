<script lang="ts">
  import type { AutosplitRoute } from "../types";

  export let sectionsGj: AutosplitRoute;
</script>

{#if sectionsGj.max_e2e_width == 0}
  <p>Route is entirely on non-arterial roads and is assumed to fit</p>
{:else}
  {#if sectionsGj.some_roads_without_sse}
    <p>Part of the route is on non-arterial roads and is assumed to fit.</p>
  {/if}

  {#if sectionsGj.min_e2e_width == sectionsGj.max_e2e_width}
    <p>
      Arterial roads have an edge-to-edge width of {sectionsGj.max_e2e_width} meters.
    </p>
  {:else}
    <p>
      Arterial roads have edge-to-edge widths between {sectionsGj.min_e2e_width}
      and
      {sectionsGj.max_e2e_width} meters.
    </p>
  {/if}

  {#if sectionsGj.cross_section_profiles.length > 1}
    <p>Cross section details vary along the route.</p>
  {/if}
  {#each sectionsGj.cross_section_profiles as profile}
    <p>{profile}</p>
  {/each}
{/if}
