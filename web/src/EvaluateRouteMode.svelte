<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import {
    MapEvents,
    GeoJSON,
    LineLayer,
    Marker,
    hoverStateFilter,
  } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { notNull } from "svelte-utils";
  import { mode, backend, routeA, routeB } from "./stores";
  import { colorByInraType } from "./common";
  import type { FeatureCollection } from "geojson";

  let gj: FeatureCollection | null = null;
  let err = "";

  async function update(
    start: { lng: number; lat: number },
    end: { lng: number; lat: number },
  ) {
    try {
      gj = await $backend!.evaluateRoute({
        start,
        end: [end.lng, end.lat],
      });
      err = "";
    } catch (error: any) {
      gj = null;
      err = error.toString();
    }
  }
  $: update($routeA!, $routeB!);

  function onRightClick(e: CustomEvent<MapMouseEvent>) {
    // Move the first marker, for convenience
    $routeA = e.detail.lngLat;
  }
</script>

<SplitComponent>
  <div slot="top">
    <button on:click={() => ($mode = { kind: "main" })}>
      Back to main mode
    </button>
  </div>

  <div slot="sidebar">
    <h2>Evaluate a route mode</h2>

    <p>
      Move the <b>A</b>
      and
      <b>B</b>
      pins to find a route. (Hint: right-click to set the first pin somewhere.)
    </p>

    {#if err}
      <p>{err}</p>
    {/if}
  </div>

  <div slot="map">
    <MapEvents on:contextmenu={onRightClick} />

    {#await notNull($backend).renderRoutes() then data}
      <GeoJSON {data}>
        <LineLayer
          id="routes"
          paint={{
            "line-width": 5,
            "line-color": colorByInraType,
          }}
        />
      </GeoJSON>
    {/await}

    {#if $routeA && $routeB}
      <Marker bind:lngLat={$routeA} draggable>
        <span class="dot">A</span>
      </Marker>
      <Marker bind:lngLat={$routeB} draggable>
        <span class="dot">B</span>
      </Marker>
    {/if}

    {#if gj}
      <GeoJSON data={gj} generateId>
        <LineLayer
          id="eval-route"
          paint={{
            "line-width": 20,
            "line-color": "red",
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
          manageHoverState
        />
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>

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
</style>
