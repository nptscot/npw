<script lang="ts">
  import type { Writable } from "svelte/store";

  export let labels: { [name: string]: string };
  export let colors: { [name: string]: string };
  export let show: Writable<{ [name: string]: boolean }>;
</script>

<ul>
  {#each Object.entries(colors) as [key, color]}
    <li style:background-color={color}>
      <label>
        <input type="checkbox" bind:checked={$show[key]} />
        {labels[key]}
      </label>
    </li>
  {/each}
</ul>

<style>
</style>
