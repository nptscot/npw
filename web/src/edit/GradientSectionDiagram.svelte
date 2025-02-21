<script lang="ts">
  import { notNull } from "svelte-utils";
  import { gradientColors } from "../colors";
  import { sum } from "../common";
  import type { AutosplitRouteByGradient } from "../types";

  export let sectionsGj: AutosplitRouteByGradient;

  $: total = sum(sectionsGj.features.map((f) => f.properties.length));
</script>

<div style="display: flex">
  {#each sectionsGj.features as f}
    <span
      style:background={gradientColors[notNull(f.properties.gradient_group)]}
      style:width={(f.properties.length / total) * 100 + "%"}
    >
      &nbsp;
    </span>
  {/each}
</div>

<style>
  span {
    border: 1px solid black;
  }
</style>
