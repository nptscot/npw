<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { LngLat, Point, type Map, type MapMouseEvent } from "maplibre-gl";
  import { onDestroy, onMount } from "svelte";
  import {
    CircleLayer,
    GeoJSON,
    LineLayer,
    MapEvents,
    Marker,
  } from "svelte-maplibre";
  import { emptyGeojson } from "svelte-utils/map";
  import { layerId } from "../common";
  import { majorJunctions } from "../layers/stores";
  import { backend } from "../stores";
  import type { Tier, Waypoint } from "../types";
  import { canStopDrawing, waypoints } from "./stores";

  export let map: Map;
  export let finish: () => void;
  export let cancel: () => void;
  export let tier: Tier;
  export let cannotUndo: boolean;

  // TODO Fix svelte-maplibre -- this isn't just a layer event
  onMount(() => {
    map.on("mouseout", onMouseOut);
  });

  onDestroy(() => {
    $waypoints = [];
    map.off("mouseout", onMouseOut);
    map.getCanvas().style.cursor = "inherit";
  });

  let undoStates: Waypoint[][] = [];
  $: if (undoStates.length == 0) {
    cannotUndo = true;
  } else {
    cannotUndo = false;
  }

  interface ExtraNode {
    point: [number, number];
    insertIdx: number;
    snapped: boolean;
  }
  let extraNodes: ExtraNode[] = [];
  $: updateExtraNodes($waypoints, draggingExtraNode);

  interface RawCursor {
    screenPt: [number, number];
    mapPt: [number, number];
  }

  let rawCursor: RawCursor | null = null;
  let previewSnappedCursor: FeatureCollection = emptyGeojson();
  let hoveringOnWaypoint = false;
  let hoveringOnExtraNode = false;
  let draggingWaypoint = false;
  let draggingExtraNode = false;

  let previewGj: FeatureCollection = emptyGeojson();
  $: updatePreview(
    $waypoints,
    rawCursor,
    hoveringOnWaypoint ||
      hoveringOnExtraNode ||
      draggingWaypoint ||
      draggingExtraNode,
  );

  $: updateCursorStyle($waypoints);
  function updateCursorStyle(waypoints: Waypoint[]) {
    let cursorStyle = waypoints.length == 0 ? "crosshair" : "inherit";
    map.getCanvas().style.cursor = cursorStyle;
  }

  export function undo() {
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

  async function onMapClick(e: CustomEvent<MapMouseEvent>) {
    captureUndoState();
    let point = await $backend!.snapPoint(
      e.detail.lngLat.toArray(),
      majorSnapThreshold(),
    );
    waypoints.update((w) => {
      w.push({
        point,
        snapped: true,
      });
      return w;
    });
  }

  async function onMouseMove(e: CustomEvent<MapMouseEvent>) {
    rawCursor = {
      screenPt: [e.detail.point.x, e.detail.point.y],
      mapPt: e.detail.lngLat.toArray(),
    };
    // Only preview where the cursor is snapping when waypoints is empty;
    // otherwise the previewed route makes the current snap point clear
    if ($waypoints.length == 0) {
      previewSnappedCursor.features = [
        {
          type: "Feature",
          properties: {},
          geometry: {
            type: "Point",
            coordinates: await $backend!.snapPoint(
              rawCursor!.mapPt,
              majorSnapThreshold(),
            ),
          },
        },
      ];
    } else {
      previewSnappedCursor.features = [];
    }
  }

  function onMouseOut() {
    rawCursor = null;
    previewGj = emptyGeojson();
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
    rawCursor: RawCursor | null,
    suppress: boolean,
  ) {
    if (suppress) {
      previewGj = emptyGeojson();
      return;
    }

    try {
      if (waypoints.length > 0 && rawCursor) {
        // Immediately show a straight line
        if (false) {
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
                    rawCursor!.mapPt,
                  ],
                },
              },
            ],
          };
        }

        // Asynchronously update to the real route (if it exists)
        previewGj = await $backend!.autosplitRoute(
          null,
          [
            waypoints[waypoints.length - 1],
            { point: rawCursor.mapPt, snapped: true },
          ],
          null,
          tier,
          majorSnapThreshold(),
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
        majorSnapThreshold(),
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

  async function finalizeDrag(node: ExtraNode) {
    draggingExtraNode = false;

    let point = await $backend!.snapPoint(node.point, majorSnapThreshold());
    waypoints.update((w) => {
      w[node.insertIdx].point = point;
      return w;
    });
  }

  function keyDown(e: KeyboardEvent) {
    let tag = (e.target as HTMLElement).tagName;
    let formFocused = tag == "INPUT" || tag == "TEXTAREA" || tag == "SELECT";

    if (e.key === "Enter" && !formFocused) {
      e.stopPropagation();
      if ($waypoints.length >= 2) {
        finish();
      }
    } else if (e.key === "Escape") {
      e.stopPropagation();
      if (canStopDrawing()) {
        cancel();
      }
    } else if (e.key == "z" && e.ctrlKey && !formFocused) {
      e.stopPropagation();
      undo();
    }
  }

  function startDraggingWaypoint() {
    captureUndoState();
    draggingWaypoint = true;
  }

  async function stopDraggingWaypoint(idx: number) {
    draggingWaypoint = false;

    let point = await $backend!.snapPoint(
      $waypoints[idx].point,
      majorSnapThreshold(),
    );
    waypoints.update((w) => {
      w[idx].point = point;
      return w;
    });
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

  let snapDistancePixels = 50.0;
  export function majorSnapThreshold(): number | null {
    if (!$majorJunctions || !rawCursor) {
      return null;
    }

    let [x, y] = rawCursor.screenPt;
    // TODO Is it a problem to only modify x?
    let nearbyPoint = new Point(x - snapDistancePixels, y);
    return map
      .unproject(nearbyPoint)
      .distanceTo(new LngLat(rawCursor.mapPt[0], rawCursor.mapPt[1]));
  }

  function debugCursor(rawCursor: RawCursor | null): FeatureCollection {
    let gj = emptyGeojson();
    if (rawCursor) {
      gj.features.push({
        type: "Feature",
        properties: {},
        geometry: {
          type: "Point",
          coordinates: rawCursor.mapPt,
        },
      });
    }
    return gj;
  }
</script>

<svelte:window on:keydown={keyDown} />

<MapEvents on:click={onMapClick} on:mousemove={onMouseMove} />

{#each extraNodes as node}
  <Marker
    draggable
    bind:lngLat={node.point}
    on:mouseenter={() => (hoveringOnExtraNode = true)}
    on:mouseleave={() => (hoveringOnExtraNode = false)}
    on:dragstart={() => addNode(node)}
    on:drag={() => updateDrag(node)}
    on:dragend={() => finalizeDrag(node)}
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
    on:dragend={() => stopDraggingWaypoint(idx)}
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

<GeoJSON data={previewSnappedCursor}>
  <CircleLayer
    {...layerId("snapper-current-snapped-waypoint")}
    paint={{
      "circle-color": "blue",
      "circle-radius": 6,
    }}
  />
</GeoJSON>

<GeoJSON data={debugCursor(rawCursor)}>
  <CircleLayer
    {...layerId("snapper-debug-cursor")}
    layout={{
      visibility: $majorJunctions && false ? "visible" : "none",
    }}
    paint={{
      "circle-color": "yellow",
      "circle-opacity": 0.5,
      "circle-radius": snapDistancePixels,
    }}
  />
</GeoJSON>

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
</style>
