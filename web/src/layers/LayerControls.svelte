<script lang="ts">
  import { allControls } from "./stores";

  export let name: string;

  let contents: HTMLDivElement | null = null;

  $: if (contents) {
    allControls.update((map) => {
      map.set(name, contents!);
      return map;
    });
  }
</script>

<div bind:this={contents}>
  <slot />
</div>
