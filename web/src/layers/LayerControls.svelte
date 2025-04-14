<script lang="ts">
  import { Checkbox } from "../common";
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
  <Checkbox small bind:checked={show}>{name}</Checkbox>

  {#if !empty}
    <div style:display={show ? "block" : "none"} style:margin-left="20px">
      <slot />
    </div>
  {/if}
</div>
