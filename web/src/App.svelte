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
  } from "svelte-utils/two_column_layout";
  import DebugNetworkMode from "./DebugNetworkMode.svelte";
  import DebugODMode from "./DebugODMode.svelte";
  import MainMode from "./MainMode.svelte";
  import Legend from "./common/Legend.svelte";
  import EditRouteMode from "./EditRouteMode.svelte";
  import EvaluateRouteMode from "./EvaluateRouteMode.svelte";
  import EvaluateODMode from "./EvaluateODMode.svelte";
  import ImportRouteMode from "./ImportRouteMode.svelte";
  import {
    map as mapStore,
    mode,
    backend,
    maptilerApiKey,
    routeSnapper,
    routeA,
    routeB,
    coherentNetwork,
    odZones,
    odPairs,
    parseOD,
  } from "./stores";
  import { Backend } from "./worker";
  import init, { JsRouteSnapper } from "route-snapper";
  import { Loading } from "svelte-utils";
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
    if (!params.has("boundary")) {
      window.alert(
        "Missing boundary param. TODO, redirect back to an LA dashboard",
      );
      return;
    }
    let boundaryName = params.get("boundary");
    loading = `Loading ${boundaryName}`;

    let backendWorker = new Backend();

    // Detect if we're running locally first
    let resp = await fetch(`${boundaryName}.bin`);
    let remote = false;
    if (resp.ok) {
      console.log(`Using locally hosted files`);
    } else {
      remote = true;
      console.log(`Using remote hosted files`);
      resp = await fetch(
        `https://assets.od2net.org/tmp_npt_editor/${boundaryName}.bin`,
      );
    }

    let bytes = await resp.arrayBuffer();
    await backendWorker.loadFile(new Uint8Array(bytes));

    // Load saved state?
    let item = window.localStorage.getItem("tmp-npt-editor");
    if (item) {
      try {
        await backendWorker.loadSavefile(item);
      } catch (err) {
        window.alert(`Couldn't restore saved state: ${err}`);
      }
    }

    let resp2 = await fetch(
      remote
        ? `https://assets.od2net.org/tmp_npt_editor/cn_manual.geojson`
        : `cn_manual.geojson`,
    );
    $coherentNetwork = await resp2.json();

    let resp3 = await fetch(
      remote
        ? `https://assets.od2net.org/tmp_npt_editor/zones.geojson`
        : `zones.geojson`,
    );
    $odZones = await resp3.json();

    let resp4 = await fetch(
      remote ? `https://assets.od2net.org/tmp_npt_editor/od.csv` : `od.csv`,
    );
    $odPairs = parseOD(await resp4.text());

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

    $routeSnapper = new JsRouteSnapper(backendWorker.toRouteSnapper());

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

  let sidebarDiv: HTMLDivElement;
  let mapDiv: HTMLDivElement;
  $: if (sidebarDiv && $sidebarContents) {
    sidebarDiv.innerHTML = "";
    sidebarDiv.appendChild($sidebarContents);
  }
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }
</script>

<Loading {loading} />

<Layout>
  <div slot="left">
    <div bind:this={sidebarDiv} />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={offlineMode
        ? "http://localhost:5173/offline/light_style.json"
        : `https://api.maptiler.com/maps/uk-openzoomstack-light/style.json?key=${maptilerApiKey}`}
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

      <Legend />

      <div bind:this={mapDiv} />

      {#if $backend}
        {#await $backend.getInvertedBoundary() then data}
          <GeoJSON {data}>
            <FillLayer paint={{ "fill-color": "black", "fill-opacity": 0.3 }} />
          </GeoJSON>
        {/await}

        {#if $mode.kind == "main"}
          <MainMode />
        {:else if $mode.kind == "import-route"}
          <ImportRouteMode />
        {:else if $mode.kind == "edit-route" && $routeSnapper && map}
          <EditRouteMode id={$mode.id} {map} routeSnapper={$routeSnapper} />
        {:else if $mode.kind == "evaluate-route"}
          <EvaluateRouteMode />
        {:else if $mode.kind == "evaluate-od"}
          <EvaluateODMode />
        {:else if $mode.kind == "debug-network"}
          <DebugNetworkMode />
        {:else if $mode.kind == "debug-od"}
          <DebugODMode />
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
