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
  label {
    display: block;
    border-radius: 0;
    padding: 2px 6px;
    text-align: left;
  }

  li {
    flex-grow: 1;
    flex-basis: 0;
    list-style: none;
  }

  li:hover {
    opacity: 0.9;
  }
</style>
