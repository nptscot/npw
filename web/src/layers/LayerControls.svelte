<script lang="ts">
  import { allControls } from "./stores";

  export let name: string;
  export let show: boolean;
  export let empty = false;

  let contents: HTMLDivElement | null = null;

  $: if (contents) {
    allControls.update((map) => {
      map.set(name, contents!);
      return map;
    });
  }
</script>

<div bind:this={contents}>
  <label>
    <input type="checkbox" bind:checked={show} />
    {name}
  </label>

  {#if !empty}
    <div
      style:display={show ? "block" : "none"}
      style:border="1px solid black"
      style:padding="4px"
      style:background="#fffff2"
    >
      <slot />
    </div>
  {/if}
</div>
