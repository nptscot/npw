<script lang="ts">
  import { referenceRoadStyle, type ReferenceRoadStyle } from "../stores";
  import { allControls } from "./stores";

  export let name: string;
  // TODO Use name for this
  export let style: ReferenceRoadStyle;
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
    <input type="radio" value={style} bind:group={$referenceRoadStyle} />
    {name}
  </label>

  {#if !empty}
    <div
      style:display={$referenceRoadStyle == style ? "block" : "none"}
      style:border="1px solid black"
      style:padding="4px"
      style:background="#fffff2"
    >
      <slot />
    </div>
  {/if}
</div>
