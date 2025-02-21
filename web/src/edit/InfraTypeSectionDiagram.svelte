<script lang="ts">
  import { notNull } from "svelte-utils";
  import { infraTypeColors } from "../colors";
  import { sum } from "../common";
  import type { AutosplitRoute } from "../types";

  export let sectionsGj: AutosplitRoute;

  $: total = sum(sectionsGj.features.map((f) => f.properties.length));
</script>

<div style="display: flex">
  {#each sectionsGj.features as f}
    {#if f.properties.kind == "new"}
      <span
        style:background={infraTypeColors[notNull(f.properties.infra_type)]}
        style:width={(f.properties.length / total) * 100 + "%"}
      >
        &nbsp;
      </span>
    {:else}
      <span style:width={(f.properties.length / total) * 100 + "%"}>
        &nbsp;
      </span>
    {/if}
  {/each}
</div>

<style>
  span {
    border: 1px solid black;
  }
</style>
