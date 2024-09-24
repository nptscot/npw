<script lang="ts">
  import { snapMode, undoLength, showAllNodes, showAllNodesGj } from "./stores";
  import { JsRouteSnapper } from "route-snapper";

  export let routeSnapper: JsRouteSnapper;

  // TODO session storage at least
  let extendRoute = false;

  // TODO When editing, we should save in the route and use the previous value
  // TODO Need to trigger redraw
  $: routeSnapper.setRouteConfig({
    avoid_doubling_back: false,
    extend_route: extendRoute,
  });

  function loadNodes(show: boolean) {
    if (show && $showAllNodesGj.features.length == 0) {
      $showAllNodesGj = JSON.parse(routeSnapper.debugSnappableNodes());
    }
  }
  $: loadNodes($showAllNodes);
</script>

<label>
  <input
    type="checkbox"
    role="switch"
    bind:checked={$snapMode}
    on:click={() => routeSnapper.toggleSnapMode()}
  />
  <u>S</u>
  nap to roads
</label>

<button
  disabled={$undoLength == 0}
  on:click={() => routeSnapper.undo()}
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
