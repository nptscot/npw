<script lang="ts">
  import { referenceRoadStyle, type ReferenceRoadStyle } from "../../stores";
  import { lastReferenceStyle } from "../stores";

  export let name: string;
  // TODO Use name for this
  export let style: ReferenceRoadStyle;
  export let empty = false;

  function toggle() {
    if ($referenceRoadStyle == style) {
      $referenceRoadStyle = "off";
      $lastReferenceStyle = style;
    } else {
      $referenceRoadStyle = style;
    }
  }
</script>

<button
  style:background={$referenceRoadStyle == style ? "green" : "grey"}
  on:click={toggle}
>
  {#if $referenceRoadStyle == "off" && $lastReferenceStyle == style}
    <b>{name}</b>
  {:else}
    {name}
  {/if}
</button>

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
