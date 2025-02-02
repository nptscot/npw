<script lang="ts">
  import type { Feature, Polygon, Position } from "geojson";
  import type { Map, MapMouseEvent } from "maplibre-gl";
  import { onDestroy } from "svelte";
  import { FillLayer, GeoJSON, LineLayer } from "svelte-maplibre";
  import { layerId } from "./";

  export let map: Map;
  export let newRectangle: (f: Feature<Polygon>) => void;

  let active = false;
  let pt1: Position | null = null;
  let pt2: Position | null = null;
  $: feature = makeRectangle(pt1, pt2);

  function makeRectangle(
    pt1: Position | null,
    pt2: Position | null,
  ): Feature<Polygon> | null {
    if (!pt1 || !pt2 || pt1 == pt2) {
      return null;
    }
    let [x1, y1] = [pt1[0], pt1[1]];
    let [x2, y2] = [pt2[0], pt2[1]];

    return {
      type: "Feature",
      properties: {},
      geometry: {
        type: "Polygon",
        coordinates: [[pt1, [x2, y1], pt2, [x1, y2], pt1]],
      },
    };
  }

  map.boxZoom.disable();
  map.on("dragstart", start);
  map.on("mousemove", move);
  map.on("mouseup", stop);
  onDestroy(() => {
    map.boxZoom.enable();
    map.dragPan.enable();
    map.off("dragstart", start);
    map.off("mousemove", move);
    map.off("mouseup", stop);
  });

  function start() {
    if (!shiftHeld) {
      return;
    }
    //e.preventDefault();
    map.dragPan.disable();
    active = true;
  }

  function move(e: MapMouseEvent) {
    if (active) {
      if (pt1) {
        pt2 = e.lngLat.toArray();
      } else {
        pt1 = e.lngLat.toArray();
      }
    }
  }

  function stop() {
    if (feature) {
      newRectangle(feature);
    }
    active = false;
    pt1 = null;
    pt2 = null;
    map.dragPan.enable();
  }

  let shiftHeld = false;
  function onKeyDown(e: KeyboardEvent) {
    shiftHeld = e.shiftKey;
  }
  function onKeyUp(e: KeyboardEvent) {
    shiftHeld = e.shiftKey;
  }
</script>

<svelte:window on:keydown={onKeyDown} on:keyup={onKeyUp} />

{#if feature}
  <GeoJSON data={feature}>
    <FillLayer
      {...layerId("draw-rectangle-fill")}
      paint={{ "fill-color": "black", "fill-opacity": 0.1 }}
    />

    <LineLayer
      {...layerId("draw-rectangle-outline")}
      paint={{
        "line-width": 1,
        "line-color": "black",
        "line-dasharray": [3, 2],
      }}
    />
  </GeoJSON>
{/if}
