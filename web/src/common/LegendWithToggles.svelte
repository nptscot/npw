<script lang="ts">
  import type { Writable } from "svelte/store";

  export let colors: { [name: string]: string };
  export let show: Writable<{ [name: string]: boolean }>;
</script>

<div style="display: flex">
  {#each Object.entries(colors) as [_, color]}
    <span style="background: {color}; width: 100%; border: 1px solid black;">
      &nbsp;
    </span>
  {/each}
</div>
<div style="display: flex; justify-content: space-between;">
  {#each Object.keys($show) as label}
    <label>
      <input type="checkbox" bind:checked={$show[label]} />
      {label}
    </label>
  {/each}
</div>
