<script lang="ts">
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
    tierColors,
  } from "../colors";
  import { sum } from "../common";
  import type { AutosplitRoute } from "../types";

  export let breakdown:
    | "infra_type"
    | "gradient"
    | "deliverability"
    | "los"
    | "tier";
  export let sectionsGj: AutosplitRoute;

  $: total = sum(sectionsGj.features.map((f) => f.properties.length));

  // TODO Slight nit: infraTypeColors[overlap] is undefined, which gives a
  // white background correctly, but is a bit messy.
</script>

<div style:display="flex" style:border="1px solid black">
  {#each sectionsGj.features as f}
    {#if breakdown == "infra_type"}
      <span
        style:background={infraTypeColors[f.properties.infra_type]}
        style:width={(f.properties.length / total) * 100 + "%"}
      >
        &nbsp;
      </span>
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
    {:else if breakdown == "tier"}
      <span
        style:background={tierColors[f.properties.tier]}
        style:width={(f.properties.length / total) * 100 + "%"}
      >
        &nbsp;
      </span>
    {/if}
  {/each}
</div>
