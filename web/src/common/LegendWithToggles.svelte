<script lang="ts">
  import type { Writable } from "svelte/store";

  export let labels: { [name: string]: string };
  export let colors: { [name: string]: string };
  export let show: Writable<{ [name: string]: boolean }>;
</script>

<div style="display: flex">
  {#each Object.entries(colors) as [key, color]}
    <span style:background={color} style:color="white">
      <label>
        <input type="checkbox" bind:checked={$show[key]} />
        {labels[key]}
      </label>
    </span>
  {/each}
</div>

<style>
  span {
    width: 100%;
    border: 1px solid black;
  }
</style>
