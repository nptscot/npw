<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import { Geocoder, emptyGeojson } from "svelte-utils/map";
  import type { Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { FillLayer, GeoJSON, MapLibre } from "svelte-maplibre";
  import {
    Layout,
    mapContents,
    leftSidebarContents,
    rightSidebarContents,
  } from "./common/layout";
  import { layerId } from "./common";
  import { getKey, getLastOpenedFileKey } from "./common/files";
  import DebugNetworkMode from "./DebugNetworkMode.svelte";
  import DebugMeshDensityMode from "./DebugMeshDensityMode.svelte";
  import MainMode from "./MainMode.svelte";
  import DisableInteractiveLayers from "./common/DisableInteractiveLayers.svelte";
  import RouteLegend from "./common/RouteLegend.svelte";
  import EditRouteMode from "./EditRouteMode.svelte";
  import EvaluateRouteMode from "./EvaluateRouteMode.svelte";
  import EvaluateODMode from "./EvaluateODMode.svelte";
  import TopBar from "./TopBar.svelte";
  import {
    map as mapStore,
    mode,
    backend,
    maptilerApiKey,
    routeA,
    routeB,
    remoteStorage,
    assetUrl,
    boundaryName,
    currentFilename,
  } from "./stores";
  import { routeTool } from "./snapper/stores";
  import { Backend } from "./worker";
  import { init, RouteTool } from "route-snapper-ts";
  import { Loading } from "svelte-utils";
  import ReferenceLayers from "./layers/ReferenceLayers.svelte";
  // TODO Indirect dependencies
  import * as pmtiles from "pmtiles";
  import maplibregl from "maplibre-gl";

  // TODO Remove later
  let offlineMode = false;
  if (offlineMode) {
    let protocol = new pmtiles.Protocol();
    maplibregl.addProtocol("pmtiles", protocol.tile);
  }

  let loading = "";

  let map: Map;
  $: if (map) {
    mapStore.set(map);
  }

  onMount(async () => {
    await init();

    let params = new URLSearchParams(window.location.search);
    $boundaryName = params.get("boundary") || "LAD_City of Edinburgh";
    loading = `Loading ${$boundaryName}`;

    let backendWorker = new Backend();

    // Detect if we're running locally first
    let resp = await fetch(`areas/${$boundaryName}.bin`);
    if (resp.ok) {
      console.log(`Using locally hosted files`);
      $remoteStorage = false;
    } else {
      console.log(`Using remote hosted files`);
      resp = await fetch(assetUrl(`areas/${$boundaryName}.bin`));
    }

    let bytes = await resp.arrayBuffer();
    try {
      await backendWorker.loadFile(new Uint8Array(bytes));
    } catch (err) {
      window.alert(`Couldn't load: ${err}`);
    }

    // Load saved state?
    let lastFile =
      params.get("file") ||
      window.localStorage.getItem(getLastOpenedFileKey($boundaryName));
    if (lastFile) {
      let item = window.localStorage.getItem(getKey($boundaryName, lastFile));
      if (item) {
        try {
          await backendWorker.loadSavefile(item);
          $currentFilename = lastFile;
        } catch (err) {
          window.alert(`Couldn't restore saved state: ${err}`);
        }
      }
    }

    loading = "";

    let bbox = await backendWorker.getBounds();
    $routeA = {
      lng: lerp(0.4, bbox[0], bbox[2]),
      lat: lerp(0.4, bbox[1], bbox[3]),
    };
    $routeB = {
      lng: lerp(0.6, bbox[0], bbox[2]),
      lat: lerp(0.6, bbox[1], bbox[3]),
    };

    // The stores are unused; the WASM API is used directly. This TS wrapper is unused.
    $routeTool = new RouteTool(
      map,
      backendWorker.toRouteSnapper(),
      writable(emptyGeojson()),
      writable(true),
      writable(0),
    );

    backend.set(backendWorker);
    await zoomToFit();
  });

  function lerp(pct: number, a: number, b: number): number {
    return a + pct * (b - a);
  }

  async function zoomToFit() {
    if (map && $backend) {
      map.fitBounds(await $backend.getBounds(), { animate: false });
    }
  }

  let leftSidebarDiv: HTMLDivElement;
  let mapDiv: HTMLDivElement;
  let rightSidebarDiv: HTMLDivElement;
  $: if (leftSidebarDiv && $leftSidebarContents) {
    leftSidebarDiv.innerHTML = "";
    leftSidebarDiv.appendChild($leftSidebarContents);
  }
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }
  $: if (rightSidebarDiv && $rightSidebarContents) {
    rightSidebarDiv.innerHTML = "";
    rightSidebarDiv.appendChild($rightSidebarContents);
  }
</script>

<svelte:head>
  <title>Network Planning Workspace - {$boundaryName}</title>
</svelte:head>

<Loading {loading} />

<Layout>
  <div slot="top">
    <TopBar />
  </div>
  <div slot="left">
    <div bind:this={leftSidebarDiv} />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100%;">
    <MapLibre
      style={offlineMode
        ? "http://localhost:5173/offline/light_style.json"
        : `https://api.maptiler.com/maps/streets-v2/style.json?key=${maptilerApiKey}`}
      standardControls
      bind:map
      on:error={(e) => {
        // @ts-expect-error ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
    >
      {#if !offlineMode}
        <Geocoder {map} apiKey={maptilerApiKey} country="gb" />
      {/if}
      <DisableInteractiveLayers />

      <RouteLegend />

      <div bind:this={mapDiv} />

      {#if $backend}
        {#await $backend.getInvertedBoundary() then data}
          <GeoJSON {data}>
            <FillLayer
              {...layerId("fade-study-area")}
              paint={{ "fill-color": "black", "fill-opacity": 0.3 }}
            />
          </GeoJSON>
        {/await}

        <ReferenceLayers />

        {#if $mode.kind == "main"}
          <MainMode />
        {:else if $mode.kind == "edit-route" && map}
          <EditRouteMode id={$mode.id} {map} />
        {:else if $mode.kind == "evaluate-route"}
          <EvaluateRouteMode prevMode={$mode.prevMode} browse={$mode.browse} />
        {:else if $mode.kind == "evaluate-od"}
          <EvaluateODMode />
        {:else if $mode.kind == "debug-network"}
          <DebugNetworkMode />
        {:else if $mode.kind == "debug-mesh-density"}
          <DebugMeshDensityMode />
        {/if}
      {/if}
    </MapLibre>
  </div>
  <div slot="right">
    <div bind:this={rightSidebarDiv} />
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
