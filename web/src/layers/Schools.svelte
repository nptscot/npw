<script lang="ts">
  import {
    GeoJSON,
    hoverStateFilter,
    CircleLayer,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { backend, type Schools } from "../stores";
  import { percent } from "../utils";
  import { QualitativeLegend } from "../common";
  import { schools as show } from "./stores";
  import type { Feature, Point, FeatureCollection } from "geojson";

  let data: Schools = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.getSchools();
    }
  }

  $: if ($show && data.features.length == 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;

  let hovered: Feature<Point, { reachable: boolean }> | null;
  let debug: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  $: updateDebug(hovered);

  async function updateDebug(
    hovered: Feature<Point, { reachable: boolean }> | null,
  ) {
    if ($backend && hovered) {
      if (hovered.properties.reachable) {
        debug = await $backend.debugReachablePath(hovered.geometry.coordinates);
      } else {
        debug = await $backend.debugUnreachablePath(
          hovered.geometry.coordinates,
        );
      }
    } else {
      debug = {
        type: "FeatureCollection",
        features: [],
      };
    }
  }
</script>

<LayerControls name="schools">
  <label>
    <input type="checkbox" bind:checked={$show} />
    Schools
  </label>

  {#if $show}
    <button class="outline" on:click={recalc}>Recalculate</button>
    <p>
      {reachable.toLocaleString()} / {data.features.length.toLocaleString()} ({percent(
        reachable,
        data.features.length,
      )}) reachable
    </p>
    <QualitativeLegend
      colors={{ Reachable: "purple", "Not reachable": "red" }}
    />
  {/if}
</LayerControls>

<GeoJSON {data} generateId>
  <CircleLayer
    manageHoverState
    paint={{
      "circle-color": ["case", ["get", "reachable"], "purple", "red"],
      "circle-radius": hoverStateFilter(5, 8),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    bind:hovered
  >
    <Popup openOn="hover" let:props>
      {props.name} is a {props.kind} school with {props.pupils} pupils. It {props.reachable
        ? "is"
        : "is not"} reachable.
    </Popup>
  </CircleLayer>
</GeoJSON>

<GeoJSON data={debug} generateId>
  <LineLayer
    paint={{
      "line-width": 3,
      "line-color": [
        "case",
        ["==", ["get", "kind"], "severance"],
        "red",
        "blue",
      ],
    }}
  />
</GeoJSON>
