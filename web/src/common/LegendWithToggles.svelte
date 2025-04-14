<script lang="ts">
  import type { Writable } from "svelte/store";

  export let labels: { [name: string]: string };
  export let colors: { [name: string]: string };
  export let show: Writable<{ [name: string]: boolean }>;

  // https://stackoverflow.com/questions/27869740/find-the-correct-wcag-aa-contrast-color-for-a-given-hex-background-value
  function contrastColor(hex: string): string {
    let r = parseInt(hex.substr(1, 2), 16);
    let g = parseInt(hex.substr(3, 2), 16);
    let b = parseInt(hex.substr(5, 2), 16);
    let brightness = (r * 299 + g * 587 + b * 114) / 1000;
    return brightness > 128 ? "#000000" : "#ffffff";
  }
</script>

<ul>
  {#each Object.entries(colors) as [key, color]}
    <li style:background-color={color} style:color={contrastColor(color)}>
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
