<script lang="ts">
  import { backgroundLayer, type BackgroundLayer } from "../../stores";
  import { lastBackgroundLayer } from "../stores";

  export let name: string;
  // TODO Use name for this
  export let style: BackgroundLayer;
  export let icon: string;
  export let relevant = false;

  function toggle() {
    if ($backgroundLayer == style) {
      $backgroundLayer = "off";
      $lastBackgroundLayer = style;
    } else {
      $backgroundLayer = style;
    }
  }
</script>

<li>
  <button
    class:selected={$backgroundLayer == style}
    class:relevant
    on:click={toggle}
  >
    <i class="fa-solid {icon}"></i>
    {#if $backgroundLayer == "off" && $lastBackgroundLayer == style}
      <b>{name}</b>
    {:else}
      {name}
    {/if}
  </button>
</li>
