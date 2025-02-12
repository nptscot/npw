<script lang="ts">
  import { referenceRoadStyle, type ReferenceRoadStyle } from "../stores";
  import { lastReferenceStyle } from "./stores";

  export let name: string;
  // TODO Use name for this
  export let style: ReferenceRoadStyle;
  export let empty = false;
</script>

<div>
  <label>
    <input type="radio" value={style} bind:group={$referenceRoadStyle} />
    {name}
    {#if style == $lastReferenceStyle && $referenceRoadStyle != $lastReferenceStyle}
      (
      <kbd>s</kbd>
      )
    {/if}
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
