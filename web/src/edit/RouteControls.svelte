<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import type { Map, MapMouseEvent } from "maplibre-gl";
  import { onDestroy } from "svelte";
  import { GeoJSON, LineLayer, MapEvents, Marker } from "svelte-maplibre";
  import { emptyGeojson } from "svelte-utils/map";
  import { stageColors, tierLabels } from "../colors";
  import { layerId } from "../common";
  import { SplitComponent } from "../common/layout";
  import { majorJunctions } from "../layers/stores";
  import { backend, currentStage } from "../stores";
  import type { Tier, Waypoint } from "../types";
  import { waypoints } from "./stores";

  export let map: Map;
  export let finish: () => void;
  export let cancel: () => void;
  export let deleteRoute: () => void;
  export let editingExisting: boolean;
  export let tier: Tier;

  onDestroy(() => {
    $waypoints = [];
    map.getCanvas().style.cursor = "inherit";
  });

  let undoStates: Waypoint[][] = [];

  interface ExtraNode {
    point: [number, number];
    insertIdx: number;
    snapped: boolean;
  }
  let extraNodes: ExtraNode[] = [];
  $: updateExtraNodes($waypoints, draggingExtraNode);

  let cursor: Waypoint | null = null;
  let hoveringOnWaypoint = false;
  let hoveringOnExtraNode = false;
  let draggingWaypoint = false;
  let draggingExtraNode = false;

  let previewGj: FeatureCollection = emptyGeojson();
  $: updatePreview(
    $waypoints,
    cursor,
    hoveringOnWaypoint ||
      hoveringOnExtraNode ||
      draggingWaypoint ||
      draggingExtraNode,
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

  function onMapClick(e: CustomEvent<MapMouseEvent>) {
    captureUndoState();
    waypoints.update((w) => {
      w.push({
        point: e.detail.lngLat.toArray(),
        snapped: true,
      });
      return w;
    });
  }

  function onMouseMove(e: CustomEvent<MapMouseEvent>) {
    cursor = {
      point: e.detail.lngLat.toArray(),
      snapped: true,
    };
  }

  // @ts-expect-error Unused, but eventually will be implemented
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
    hoveringOnWaypoint = false;
  }

  async function updatePreview(
    waypoints: Waypoint[],
    cursor: Waypoint | null,
    suppress: boolean,
  ) {
    if (suppress) {
      previewGj = emptyGeojson();
      return;
    }

    try {
      if (waypoints.length > 0 && cursor) {
        // Immediately show a straight line
        previewGj = {
          type: "FeatureCollection",
          features: [
            {
              type: "Feature",
              properties: {},
              geometry: {
                type: "LineString",
                coordinates: [
                  waypoints[waypoints.length - 1].point,
                  cursor.point,
                ],
              },
            },
          ],
        };

        // Asynchronously update to the real route (if it exists)
        previewGj = await $backend!.autosplitRoute(
          null,
          [waypoints[waypoints.length - 1], cursor],
          null,
          tier,
          $majorJunctions,
        );
      }
    } catch (err) {}
  }

  async function updateExtraNodes(
    waypoints: Waypoint[],
    draggingExtraNode: boolean,
  ) {
    if (draggingExtraNode) {
      return;
    }

    let nodes: ExtraNode[] = [];
    let insertIdx = 1;

    for (let i = 0; i < waypoints.length - 1; i++) {
      let extra = await $backend!.getExtraNodes(
        waypoints[i],
        waypoints[i + 1],
        $majorJunctions,
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
    hoveringOnExtraNode = false;
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
      if ($waypoints.length >= 2) {
        finish();
      }
    } else if (e.key === "Escape") {
      e.stopPropagation();
      cancel();
    } else if (e.key == "z" && e.ctrlKey) {
      e.stopPropagation();
      undo();
    }
  }

  function startDraggingWaypoint() {
    captureUndoState();
    draggingWaypoint = true;
  }

  function onClickWaypoint(idx: number) {
    if ($waypoints.length < 2) {
      return;
    }
    // Click the end to finish
    if (idx == $waypoints.length - 1) {
      finish();
    }
  }

  // @ts-expect-error Need to write a proper type for this
  $: headerLabel = { ...tierLabels, assessment: "Assess" }[$currentStage];

  // @ts-expect-error TS doesn't understand the ... syntax?
  $: labelColor = stageColors[$currentStage];
</script>

<svelte:window on:keydown={keyDown} />

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <span
          class="ds_page-header__label ds_content-label"
          style:color={labelColor}
        >
          {headerLabel}
        </span>

        {#if editingExisting}
          <h2 class="ds_page-header__title">Edit a route</h2>
        {:else}
          <h2 class="ds_page-header__title">Draw a new route</h2>
        {/if}
      </header>

      {#if $currentStage == "Primary" || $currentStage == "Secondary"}
        <label>
          <input type="checkbox" bind:checked={$majorJunctions} />
          Snap to main roads
        </label>
      {/if}

      {#if $waypoints.length == 0}
        <p>Click to set the start of the route.</p>
      {:else if $waypoints.length == 1}
        <p>Click to set the end of the route.</p>
      {:else}
        <p>
          Click to extend the route, drag points to adjust, or change the route
          properties.
        </p>

        <div class="ds_button-group">
          <button class="ds_button" on:click={finish}>Finish</button>
          <span>or</span>
          <button class="ds_button ds_button--cancel" on:click={cancel}>
            Cancel
          </button>
        </div>

        {#if editingExisting}
          <div>
            <button
              class="ds_button ds_button--secondary"
              on:click={deleteRoute}
            >
              Delete
            </button>
          </div>
        {/if}

        <div>
          <button
            class="ds_button ds_button--secondary"
            disabled={undoStates.length == 0}
            on:click={undo}
            title="Undo last change on the map"
          >
            Undo
          </button>
        </div>
      {/if}

      <slot name="extra-controls" />
    </div>
  </div>

  <div slot="map">
    <MapEvents on:click={onMapClick} on:mousemove={onMouseMove} />

    {#each extraNodes as node}
      <Marker
        draggable
        bind:lngLat={node.point}
        on:mouseenter={() => (hoveringOnExtraNode = true)}
        on:mouseleave={() => (hoveringOnExtraNode = false)}
        on:dragstart={() => addNode(node)}
        on:drag={() => updateDrag(node)}
        on:dragend={finalizeDrag}
        zIndex={0}
      >
        <span class="extra-node-clickable" class:hide={draggingExtraNode}>
          <span class="extra-node-display" class:hide={draggingExtraNode} />
        </span>
      </Marker>
    {/each}

    {#each $waypoints as waypt, idx}
      <Marker
        draggable
        bind:lngLat={waypt.point}
        on:click={() => onClickWaypoint(idx)}
        on:contextmenu={() => removeWaypoint(idx)}
        on:mouseenter={() => (hoveringOnWaypoint = true)}
        on:mouseleave={() => (hoveringOnWaypoint = false)}
        on:dragstart={startDraggingWaypoint}
        on:dragend={() => (draggingWaypoint = false)}
        zIndex={1}
      >
        <span class="waypoint">{idx + 1}</span>
      </Marker>
    {/each}

    <GeoJSON data={previewGj}>
      <LineLayer
        {...layerId("snapper-preview")}
        paint={{
          "line-color": "blue",
          "line-width": 3,
          "line-dasharray": [3, 2],
        }}
      />
    </GeoJSON>

    <slot name="extra-map" />
  </div>
</SplitComponent>

<style>
  /** Styling on the map **/
  .waypoint {
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

  .waypoint:hover {
    border: 1px solid black;
    cursor: pointer;
  }

  .extra-node-clickable {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    display: flex;
    position: relative;
  }

  .extra-node-clickable:hover {
    background-color: blue;
    width: 20px;
    height: 20px;
  }

  .extra-node-display {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    display: flex;
    background-color: white;

    /* Center the small displayed circle inside the larger invisible hitbox circle */
    position: absolute;
    top: 50%;
    left: 50%;
    margin: -2.5px 0px 0px -2.5px;
  }

  .hide {
    visibility: hidden;
  }

  /** Controls **/
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
