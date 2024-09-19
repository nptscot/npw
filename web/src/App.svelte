<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import { Geocoder } from "svelte-utils/map";
  import type { Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { FillLayer, GeoJSON, MapLibre } from "svelte-maplibre";
  import {
    Layout,
    mapContents,
    sidebarContents,
    topContents,
  } from "svelte-utils/top_bar_layout";
  import DebugMode from "./DebugMode.svelte";
  import MainMode from "./MainMode.svelte";
  import SketchRouteMode from "./SketchRouteMode.svelte";
  import RouteDetailsMode from "./RouteDetailsMode.svelte";
  import EvaluateRouteMode from "./EvaluateRouteMode.svelte";
  import {
    map as mapStore,
    mode,
    backend,
    maptilerApiKey,
    routeTool,
    routeA,
    routeB,
  } from "./stores";
  import { Backend } from "./worker";
  import { routeToolGj, snapMode, undoLength } from "./snapper/stores";
  import { init, RouteTool } from "route-snapper-ts";
  // TODO Indirect dependencies
  import * as pmtiles from "pmtiles";
  import maplibregl from "maplibre-gl";

  // TODO Remove later
  let offlineMode = true;
  if (offlineMode) {
    let protocol = new pmtiles.Protocol();
    maplibregl.addProtocol("pmtiles", protocol.tile);
  }

  let map: Map;
  $: if (map) {
    mapStore.set(map);
  }

  $: if (map && $backend) {
    $routeTool = new RouteTool(
      map,
      $backend.toRouteSnapper(),
      routeToolGj,
      snapMode,
      undoLength,
    );
  }

  onMount(async () => {
    await init();

    let backendWorker = new Backend();

    let resp = await fetch("model.bin");
    let bytes = await resp.arrayBuffer();
    await backendWorker.loadFile(new Uint8Array(bytes));

    backend.set(backendWorker);

    await zoomToFit();

    let bbox = await backendWorker.getBounds();
    $routeA = {
      lng: lerp(0.4, bbox[0], bbox[2]),
      lat: lerp(0.4, bbox[1], bbox[3]),
    };
    $routeB = {
      lng: lerp(0.6, bbox[0], bbox[2]),
      lat: lerp(0.6, bbox[1], bbox[3]),
    };
  });

  function lerp(pct: number, a: number, b: number): number {
    return a + pct * (b - a);
  }

  async function zoomToFit() {
    if (map && $backend) {
      map.fitBounds(await $backend.getBounds(), { animate: false });
    }
  }

  let topDiv: HTMLSpanElement;
  let sidebarDiv: HTMLDivElement;
  let mapDiv: HTMLDivElement;
  $: if (topDiv && $topContents) {
    topDiv.innerHTML = "";
    topDiv.appendChild($topContents);
  }
  $: if (sidebarDiv && $sidebarContents) {
    sidebarDiv.innerHTML = "";
    sidebarDiv.appendChild($sidebarContents);
  }
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }
</script>

<Layout>
  <div slot="top" style="display: flex">
    Logo/about
    <span bind:this={topDiv} style="width: 100%" />
  </div>
  <div slot="left">
    <div bind:this={sidebarDiv} />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100%;">
    <MapLibre
      style={offlineMode
        ? "http://localhost:5173/offline/light_style.json"
        : `https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerApiKey}`}
      standardControls
      hash
      bind:map
      on:error={(e) => {
        // @ts-expect-error ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
    >
      {#if !offlineMode}
        <Geocoder {map} apiKey={maptilerApiKey} />
      {/if}
      <div bind:this={mapDiv} />

      {#if $backend}
        {#await $backend.getInvertedBoundary() then data}
          <GeoJSON {data}>
            <FillLayer paint={{ "fill-color": "black", "fill-opacity": 0.3 }} />
          </GeoJSON>
        {/await}

        {#if $mode.kind == "main"}
          <MainMode />
        {:else if $mode.kind == "sketch-route"}
          <SketchRouteMode id={$mode.id} />
        {:else if $mode.kind == "route-details"}
          <RouteDetailsMode id={$mode.id} />
        {:else if $mode.kind == "evaluate-route"}
          <EvaluateRouteMode />
        {:else if $mode.kind == "debug"}
          <DebugMode />
        {/if}
      {/if}
    </MapLibre>
  </div>
</Layout>

<style>
  :global(.maplibregl-popup-content) {
    background-color: var(--pico-background-color);
  }

  /* picocss messes up maplibre controls; workaround */
  :global(.maplibregl-ctrl > button) {
    margin-bottom: 0px;
  }
</style>
