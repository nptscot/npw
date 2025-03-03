<script lang="ts">
  import { referenceRoadStyle, type ReferenceRoadStyle } from "../../stores";
  import { lastReferenceStyle } from "../stores";

  export let name: string;
  // TODO Use name for this
  export let style: ReferenceRoadStyle;

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
  style:font-size="20px"
  on:click={toggle}
>
  {#if $referenceRoadStyle == "off" && $lastReferenceStyle == style}
    <b>{name}</b>
  {:else}
    {name}
  {/if}
</button>
