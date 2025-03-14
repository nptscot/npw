<script lang="ts">
  import { notNull, QualitativeLegend } from "svelte-utils";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
  } from "../colors";
  import { sum } from "../common";
  import type { AutosplitRoute } from "../types";

  export let breakdown: "infra_type" | "gradient" | "deliverability" | "los";
  export let sectionsGj: AutosplitRoute;

  $: total = sum(sectionsGj.features.map((f) => f.properties.length));
</script>

{#if breakdown == "infra_type"}
  <QualitativeLegend colors={infraTypeColors} />
{:else if breakdown == "gradient"}
  <QualitativeLegend colors={gradientColors} horiz />
{:else if breakdown == "los"}
  <QualitativeLegend colors={levelOfServiceColors} horiz />
{/if}

<div style:display="flex" style:border="1px solid black">
  {#each sectionsGj.features as f}
    {#if breakdown == "infra_type"}
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
    {:else if breakdown == "gradient"}
      <span
        style:background={gradientColors[f.properties.gradient_group]}
        style:width={(f.properties.length / total) * 100 + "%"}
      >
        &nbsp;
      </span>
    {:else if breakdown == "deliverability"}
      <span
        style:background={f.properties.fits ? "green" : "red"}
        style:width={(f.properties.length / total) * 100 + "%"}
      >
        &nbsp;
      </span>
    {:else if breakdown == "los"}
      <span
        style:background={levelOfServiceColors[f.properties.los]}
        style:width={(f.properties.length / total) * 100 + "%"}
      >
        &nbsp;
      </span>
    {/if}
  {/each}
</div>
