<script lang="ts">
  import { roadStyle, type RoadStyle } from "../stores";
  import { allControls } from "./stores";

  export let name: string;
  // TODO Use name for this
  export let style: RoadStyle;

  let contents: HTMLDivElement | null = null;

  $: if (contents) {
    allControls.update((map) => {
      map.set(name, contents!);
      return map;
    });
  }

  function toggle() {
    $roadStyle = $roadStyle == style ? "off" : style;
  }
</script>

<div bind:this={contents}>
  <label>
    <input type="checkbox" checked={$roadStyle == style} on:change={toggle} />
    {name}
  </label>

  <div
    style:display={$roadStyle == style ? "block" : "none"}
    style:border="1px solid black"
    style:padding="4px"
    style:background="#fffff2"
  >
    <slot />
  </div>
</div>
