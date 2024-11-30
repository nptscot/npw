<script lang="ts">
  import { HelpButton } from "../common";
  import { routeTool, waypoints, type Waypoint } from "./stores";
  import { Marker, MapEvents, GeoJSON, LineLayer } from "svelte-maplibre";
  import type { MapMouseEvent, Map } from "maplibre-gl";
  import type { FeatureCollection } from "geojson";
  import { RouteTool } from "route-snapper-ts";
  import { onDestroy } from "svelte";
  import { emptyGeojson } from "svelte-utils/map";

  export let map: Map;

  export let finish: () => void;
  export let cancel: () => void;

  onDestroy(() => {
    $waypoints = [];
    $routeTool?.stop();
    map.getCanvas().style.cursor = "inherit";
  });

  let drawMode: "append-start" | "append-end" | "adjust" = "append-end";
  let snapMode = true;
  let undoStates: Waypoint[][] = [];

  interface ExtraNode {
    point: [number, number];
    insertIdx: number;
    snapped: boolean;
  }
  let extraNodes: ExtraNode[] = [];
  $: updateExtraNodes($routeTool, $waypoints, drawMode, draggingExtraNode);

  let cursor: Waypoint | null = null;
  let hoveringOnMarker = false;
  let draggingMarker = false;
  let draggingExtraNode = false;
  $: previewGj = getPreview(
    $routeTool,
    $waypoints,
    drawMode,
    cursor,
    hoveringOnMarker || draggingMarker,
  );

  $: updateCursor($waypoints);
  function updateCursor(waypoints: Waypoint[]) {
    let cursor = waypoints.length == 0 ? "crosshair" : "inherit";
    map.getCanvas().style.cursor = cursor;
  }

  function undo() {
    let state = undoStates.pop();
    undoStates = undoStates;
    if (state) {
      $waypoints = state;
    }
  }

  function captureUndoState() {
    if (undoStates.length == 100) {
      undoStates.shift();
    }
    undoStates = [...undoStates, JSON.parse(JSON.stringify($waypoints))];
  }

  function toggleSnap() {
    snapMode = !snapMode;
    if (cursor) {
      cursor.snapped = snapMode;
    }
  }

  function onMapClick(e: CustomEvent<MapMouseEvent>) {
    captureUndoState();
    waypoints.update((w) => {
      if (drawMode == "append-start") {
        w.splice(0, 0, {
          point: e.detail.lngLat.toArray(),
          snapped: snapMode,
        });
      } else if (drawMode == "append-end") {
        w.push({
          point: e.detail.lngLat.toArray(),
          snapped: snapMode,
        });
      }
      return w;
    });
  }

  function onMouseMove(e: CustomEvent<MapMouseEvent>) {
    cursor = {
      point: e.detail.lngLat.toArray(),
      snapped: snapMode,
    };
  }

  function toggleSnapped(idx: number) {
    captureUndoState();
    waypoints.update((w) => {
      w[idx].snapped = !w[idx].snapped;
      return w;
    });
  }

  function removeWaypoint(idx: number) {
    captureUndoState();
    waypoints.update((w) => {
      w.splice(idx, 1);
      return w;
    });
    hoveringOnMarker = false;
  }

  function calculateRoutes(
    routeTool: RouteTool | null,
    waypoints: Waypoint[],
  ): FeatureCollection {
    try {
      if (routeTool) {
        return JSON.parse(routeTool.inner.calculateRoute(waypoints));
      }
    } catch (err) {}
    return emptyGeojson();
  }

  function getPreview(
    routeTool: RouteTool | null,
    waypoints: Waypoint[],
    drawMode: "append-start" | "append-end" | "adjust",
    cursor: Waypoint | null,
    suppress: boolean,
  ): FeatureCollection {
    if (suppress) {
      return emptyGeojson();
    }
    try {
      if (routeTool && waypoints.length > 0 && cursor) {
        if (drawMode == "append-start") {
          return JSON.parse(
            routeTool.inner.calculateRoute([cursor, waypoints[0]]),
          );
        } else if (drawMode == "append-end") {
          return JSON.parse(
            routeTool.inner.calculateRoute([
              waypoints[waypoints.length - 1],
              cursor,
            ]),
          );
        }
      }
    } catch (err) {}
    return emptyGeojson();
  }

  function updateExtraNodes(
    routeTool: RouteTool | null,
    waypoints: Waypoint[],
    drawMode: "append-start" | "append-end" | "adjust",
    draggingExtraNode: boolean,
  ) {
    if (draggingExtraNode) {
      return;
    }
    if (!routeTool || drawMode != "adjust") {
      extraNodes = [];
      return;
    }

    let nodes: ExtraNode[] = [];
    let insertIdx = 1;

    for (let i = 0; i < waypoints.length - 1; i++) {
      let extra = JSON.parse(
        routeTool.inner.getExtraNodes(waypoints[i], waypoints[i + 1]),
      );
      for (let [x, y, snapped] of extra) {
        nodes.push({ point: [x, y], snapped, insertIdx });
      }
      insertIdx++;
    }

    extraNodes = nodes;
  }

  function addNode(node: ExtraNode) {
    // Turn an extra node into a waypoint. Capture state before the user drags
    // around the new waypoint.
    captureUndoState();
    waypoints.update((w) => {
      w.splice(node.insertIdx, 0, {
        point: node.point,
        snapped: node.snapped,
      });
      return w;
    });
    draggingExtraNode = true;
  }

  function updateDrag(node: ExtraNode) {
    // Don't constantly update undoStates
    waypoints.update((w) => {
      w[node.insertIdx].point = node.point;
      return w;
    });
  }

  function finalizeDrag() {
    draggingExtraNode = false;
  }

  function keyDown(e: KeyboardEvent) {
    let tag = (e.target as HTMLElement).tagName;
    let formFocused = tag == "INPUT" || tag == "TEXTAREA";

    if (e.key === "Enter" && !formFocused) {
      e.stopPropagation();
      finish();
    } else if (e.key === "Escape") {
      e.stopPropagation();
      cancel();
    } else if (e.key == "s" && !formFocused) {
      toggleSnap();
    } else if (e.key == "1" && !formFocused) {
      drawMode = "append-start";
    } else if (e.key == "2" && !formFocused) {
      drawMode = "append-end";
    } else if (e.key == "3" && !formFocused) {
      drawMode = "adjust";
    } else if (e.key == "z" && e.ctrlKey) {
      e.stopPropagation();
      undo();
    }
  }

  function startDraggingWaypoint() {
    captureUndoState();
    draggingMarker = true;
  }
</script>

<div style="display: flex">
  <div style="display: flex; flex-direction: row">
    <fieldset>
      <label>
        <input
          type="radio"
          value="snap"
          bind:group={snapMode}
          on:change={toggleSnap}
        />
        Snap to roads
      </label>
      <label>
        <input
          type="radio"
          value="free"
          bind:group={snapMode}
          on:change={toggleSnap}
        />
        Draw anywhere
      </label>
    </fieldset>

    <fieldset>
      <label>
        <input type="radio" value="append-start" bind:group={drawMode} />
        Extend from start
      </label>
      <label>
        <input type="radio" value="append-end" bind:group={drawMode} />
        Extend from end
      </label>
      <label>
        <input type="radio" value="adjust" bind:group={drawMode} />
        Adjust middle points
      </label>
    </fieldset>
  </div>

  <div style="margin-left: auto">
    <div role="group">
      <button on:click={finish} disabled={$waypoints.length < 2}>Finish</button>
      <button
        class="secondary"
        disabled={undoStates.length == 0}
        on:click={undo}
      >
        {#if undoStates.length == 0}
          Undo
        {:else}
          Undo ({undoStates.length})
        {/if}
      </button>
      <button class="secondary" on:click={cancel}>Cancel</button>
      <HelpButton>
        <ul>
          <li>
            <b>Click</b>
            the map to add new points, while extending from the start or end
          </li>
          <li>
            <b>Click and drag</b>
            any point to move it
          </li>
          <li>
            <b>Click</b>
            a waypoint to toggle snapping
          </li>
          <li>
            <b>Right click</b>
            a waypoint to delete it
          </li>
        </ul>

        <p>Keyboard shortcuts:</p>
        <ul>
          <li>
            <b>1</b>
            to extend from start
          </li>
          <li>
            <b>2</b>
            to extend from end
          </li>
          <li>
            <b>3</b>
            to drag middle points
          </li>
          <li>
            <b>s</b>
            to switch between snapping to roads and drawing anywhere
          </li>
          <li>
            <b>Control+Z</b>
            to undo your last change
          </li>
          <li>
            <b>Enter</b>
            to finish
          </li>
          <li>
            <b>Escape</b>
            to cancel
          </li>
        </ul>
      </HelpButton>
    </div>
  </div>
</div>

<svelte:window on:keydown={keyDown} />

<MapEvents on:click={onMapClick} on:mousemove={onMouseMove} />

{#each extraNodes as node}
  <Marker
    draggable
    bind:lngLat={node.point}
    on:dragstart={() => addNode(node)}
    on:drag={() => updateDrag(node)}
    on:dragend={finalizeDrag}
    zIndex={0}
  >
    <span
      class="dot"
      class:snapped-node={node.snapped}
      class:free-node={!node.snapped}
      class:hide={draggingExtraNode}
    />
  </Marker>
{/each}

{#each $waypoints as waypt, idx}
  <Marker
    draggable
    bind:lngLat={waypt.point}
    on:click={() => toggleSnapped(idx)}
    on:contextmenu={() => removeWaypoint(idx)}
    on:mouseenter={() => (hoveringOnMarker = true)}
    on:mouseleave={() => (hoveringOnMarker = false)}
    on:dragstart={startDraggingWaypoint}
    on:dragend={() => (draggingMarker = false)}
    zIndex={1}
  >
    <span class="dot" class:snapped={waypt.snapped}>{idx + 1}</span>
  </Marker>
{/each}

<GeoJSON data={calculateRoutes($routeTool, $waypoints)} generateId>
  <LineLayer
    paint={{
      "line-color": "black",
      "line-width": 10,
    }}
  />
</GeoJSON>

<GeoJSON data={previewGj}>
  <LineLayer
    paint={{
      "line-color": "black",
      "line-width": 3,
    }}
  />
</GeoJSON>

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;

    color: white;
    background-color: blue;
    font-weight: bold;
  }

  .dot:hover {
    border: 1px solid black;
    cursor: pointer;
  }

  .snapped {
    background-color: red;
  }

  .free-node,
  .snapped-node {
    width: 20px;
    height: 20px;
    background-color: grey;
  }

  .snapped-node:hover {
    border: 3px solid red;
  }

  .free-node:hover {
    border: 3px solid blue;
  }

  .hide {
    visibility: hidden;
  }
</style>
