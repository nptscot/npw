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
    <div style:display={show ? "block" : "none"} style:margin-left="20px">
      <slot />
    </div>
  {/if}
</div>
