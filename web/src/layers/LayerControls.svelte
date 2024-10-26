<script lang="ts">
  import { allControls } from "./stores";

  let contents: HTMLDivElement | null = null;

  $: if (contents) {
    allControls.update((list) => {
      list.push(contents!);
      return list;
    });
  }
</script>

<div bind:this={contents}>
  <slot />
</div>
