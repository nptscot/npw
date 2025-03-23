<script lang="ts">
  import { referenceRoadStyle, type ReferenceRoadStyle } from "../../stores";
  import { lastReferenceStyle } from "../stores";

  export let name: string;
  // TODO Use name for this
  export let style: ReferenceRoadStyle;
  export let icon: string;
  export let relevant = false;

  function toggle() {
    if ($referenceRoadStyle == style) {
      $referenceRoadStyle = "off";
      $lastReferenceStyle = style;
    } else {
      $referenceRoadStyle = style;
    }
  }
</script>

<li>
  <button
    class:selected={$referenceRoadStyle == style}
    class:relevant
    on:click={toggle}
  >
    <i class="fa-solid {icon}"></i>
    {#if $referenceRoadStyle == "off" && $lastReferenceStyle == style}
      <b>{name}</b>
    {:else}
      {name}
    {/if}
  </button>
</li>
