<script lang="ts">
  import { snapMode, undoLength, showAllNodes, showAllNodesGj } from "./stores";
  import { RouteTool } from "route-snapper-ts";

  export let routeTool: RouteTool;

  // TODO session storage at least
  let extendRoute = false;

  // TODO When editing, we should save in the route and use the previous value
  $: routeTool.setRouteConfig({
    avoid_doubling_back: false,
    extend_route: extendRoute,
  });

  function loadNodes(show: boolean) {
    if (show && $showAllNodesGj.features.length == 0) {
      $showAllNodesGj = JSON.parse(routeTool.inner.debugSnappableNodes());
    }
  }
  $: loadNodes($showAllNodes);
</script>

<div>
  <button on:click={() => routeTool.finish()}>Finish</button>
  <button on:click={() => routeTool.cancel()}>Cancel</button>
</div>

<button
  disabled={$undoLength == 0}
  on:click={() => routeTool.undo()}
  data-tooltip="Ctrl+Z"
>
  {#if $undoLength == 0}
    Undo
  {:else}
    Undo ({$undoLength})
  {/if}
</button>

<label>
  <input type="checkbox" bind:checked={extendRoute} />
  Add points to end
</label>
<label>
  <input type="checkbox" bind:checked={$showAllNodes} />
  Show all snappable points
</label>

{#if $snapMode}
  <p style="background: red; color: white; padding: 8px;">
    Snapping to existing roads. Press <b>s</b>
    or click below to draw anywhere
  </p>
  <button on:click={() => routeTool.toggleSnapMode()}>Draw anywhere</button>
{:else}
  <p style="background: blue; color: white; padding: 8px;">
    Drawing points anywhere. Press <b>s</b>
    or click below to snap to roads
  </p>
  <button on:click={() => routeTool.toggleSnapMode()}>Snap to roads</button>
{/if}

<ul>
  <li>
    <b>Click</b>
    the map to add points
  </li>
  <li>
    Press <b>s</b>
    to switch between snapping points to existing roads and drawing anywhere
  </li>
  <li>
    <b>Click and drag</b>
    any point to move it
  </li>
  <li>
    <b>Click</b>
    a waypoint to delete it
  </li>
  <li>
    Press <b>Control+Z</b>
    to undo your last change
  </li>
  <li>
    Press <b>Enter</b>
    or
    <b>double click</b>
    to finish
  </li>
  <li>
    Press <b>Escape</b>
    to cancel
  </li>
</ul>
