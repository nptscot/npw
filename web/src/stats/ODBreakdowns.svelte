<script lang="ts">
  import type { ODStats } from "../stores";

  export let od: ODStats;
</script>

<details>
  <summary>Percent of demand by infrastructure type</summary>
  <ul>
    {#each Object.entries(od.od_percents_infra_type).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
      <li>{key}: {Math.round(pct * 100)}%</li>
    {/each}
  </ul>
</details>

<details>
  <summary>Percent of demand by level of service:</summary>
  <ul>
    {#each Object.entries(od.od_percents_los).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
      <li>{key}: {Math.round(pct * 100)}%</li>
    {/each}
  </ul>
</details>
