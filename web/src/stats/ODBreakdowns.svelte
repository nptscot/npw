<script lang="ts">
  import { infraTypeColors, levelOfServiceColors, tierColors } from "../colors";
  import type { ODStats } from "../stores";

  export let od: ODStats;
</script>

<details>
  <summary>Percent of demand by infrastructure type</summary>
  <ul>
    {#each Object.entries(od.od_percents_infra_type).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
      <li>
        <span style:color={infraTypeColors[key] || "black"}>{key}</span>
        : {Math.round(pct * 100)}%
      </li>
    {/each}
  </ul>
</details>

<details>
  <summary>Percent of demand by tier</summary>
  <ul>
    {#each Object.entries(od.od_percents_tier).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
      <li>
        <span style:color={tierColors[key] || "black"}>{key}</span>
        : {Math.round(pct * 100)}%
      </li>
    {/each}
  </ul>
</details>

<details>
  <summary>Percent of demand by level of service:</summary>
  <ul>
    {#each Object.entries(od.od_percents_los).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
      <li>
        <span style:color={levelOfServiceColors[key] || "black"}>{key}</span>
        : {Math.round(pct * 100)}%
      </li>
    {/each}
  </ul>
</details>
