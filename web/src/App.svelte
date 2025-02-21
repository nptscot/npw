<script lang="ts">
  import * as Comlink from "comlink";
  import Stats from "./stats/Stats.svelte";
  import "@picocss/pico/css/pico.conditional.jade.min.css";
  import type { Map } from "maplibre-gl";
  import maplibregl from "maplibre-gl";
  // TODO Indirect dependencies
  import * as pmtiles from "pmtiles";
  import { onMount } from "svelte";
  import {
    FillLayer,
    GeoJSON,
    MapLibre,
    NavigationControl,
    ScaleControl,
  } from "svelte-maplibre";
  import { fetchWithProgress, Loading } from "svelte-utils";
  import { Geocoder } from "svelte-utils/map";
  import hospital1Icon from "../assets/hospital_reachable.png";
  import hospital2Icon from "../assets/hospital_unreachable.png";
  import school1Icon from "../assets/school_reachable.png";
  import school2Icon from "../assets/school_unreachable.png";
  import BulkEditMode from "./BulkEditMode.svelte";
  import { layerId } from "./common";
  import DisableInteractiveLayers from "./common/DisableInteractiveLayers.svelte";
  import { getKey, getLastOpenedFileKey } from "./common/files";
  import { Layout, leftSidebarContents, mapContents } from "./common/layout";
  import StreetView from "./common/StreetView.svelte";
  import EditRouteMode from "./edit/EditRouteMode.svelte";
  import EvaluateJourneyMode from "./EvaluateJourneyMode.svelte";
  import ReferenceLayers from "./layers/ReferenceLayers.svelte";
  import PickReferenceStyle from "./layers/roads/PickReferenceStyle.svelte";
  import MainMode from "./MainMode.svelte";
  import {
    assetUrl,
    backend,
    boundaryName,
    currentFilename,
    devMode,
    map as mapStore,
    maptilerApiKey,
    mode,
    remoteStorage,
    routeA,
    routeB,
  } from "./stores";
  import TopBar from "./TopBar.svelte";
  import type { Backend } from "./worker";
  import workerWrapper from "./worker?worker";

  // TODO Remove later
  let offlineMode = false;
  if (offlineMode) {
    let protocol = new pmtiles.Protocol();
    maplibregl.addProtocol("pmtiles", protocol.tile);
  }

  let loading = "";
  let progress = 0;

  let map: Map;
  $: if (map) {
    mapStore.set(map);
  }

  onMount(async () => {
    let params = new URLSearchParams(window.location.search);
    $boundaryName = params.get("boundary") || "LAD_City of Edinburgh";
    loading = `Loading ${$boundaryName}`;

    interface WorkerConstructor {
      new (): Backend;
    }

    let MyWorker: Comlink.Remote<WorkerConstructor> = Comlink.wrap(
      new workerWrapper(),
    );
    let backendWorker = await new MyWorker();

    // Detect if we're running locally first
    let bytes: Uint8Array<ArrayBufferLike> = new Uint8Array();
    try {
      bytes = await fetchWithProgress(`areas/${$boundaryName}.bin`, (p) => {
        progress = p;
      });
      console.log(`Using locally hosted files`);
      $remoteStorage = false;
    } catch (err) {
      console.log(`Using remote hosted files`);
      bytes = await fetchWithProgress(
        assetUrl(`areas/${$boundaryName}.bin`),
        (p) => {
          progress = p;
        },
      );
    }

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
          window.location.href = "index.html";
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
  $: if (leftSidebarDiv && $leftSidebarContents) {
    leftSidebarDiv.innerHTML = "";
    leftSidebarDiv.appendChild($leftSidebarContents);
  }
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }
</script>

<svelte:head>
  <title>Network Planning Workspace - {$boundaryName}</title>
</svelte:head>

<Loading {loading} {progress} />

<Layout>
  <div slot="top" class="pico">
    <TopBar />
  </div>
  <div slot="left" class="pico">
    <div bind:this={leftSidebarDiv} />
    <hr />

    <label>
      <input type="checkbox" bind:checked={$devMode} />
      Dev mode
    </label>
  </div>

  <div slot="main" style="position:relative; width: 100%; height: 100%;">
    <MapLibre
      style={offlineMode
        ? "http://localhost:5173/offline/light_style.json"
        : `https://api.maptiler.com/maps/streets-v2/style.json?key=${maptilerApiKey}`}
      bind:map
      on:error={(e) => {
        // @ts-expect-error ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
      images={[
        { id: "school_reachable", url: school1Icon },
        { id: "school_unreachable", url: school2Icon },
        { id: "hospital_reachable", url: hospital1Icon },
        { id: "hospital_unreachable", url: hospital2Icon },
      ]}
      hash
    >
      <NavigationControl />
      <ScaleControl />
      {#if !offlineMode}
        <Geocoder {map} apiKey={maptilerApiKey} country="gb" />
      {/if}
      <DisableInteractiveLayers />
      <StreetView />

      <div bind:this={mapDiv} />

      {#if $backend}
        <Stats />

        {#await $backend.getInvertedBoundary() then data}
          <GeoJSON {data}>
            <FillLayer
              {...layerId("fade-study-area")}
              paint={{ "fill-color": "black", "fill-opacity": 0.3 }}
            />
          </GeoJSON>
        {/await}

        <ReferenceLayers />
        <PickReferenceStyle />

        {#if $mode.kind == "main"}
          <MainMode />
        {:else if $mode.kind == "edit-route" && map}
          <EditRouteMode id={$mode.id} {map} />
        {:else if $mode.kind == "evaluate-journey"}
          <EvaluateJourneyMode
            prevMode={$mode.prevMode}
            browse={$mode.browse}
          />
        {:else if $mode.kind == "bulk-edit"}
          <BulkEditMode />
        {/if}
      {/if}
    </MapLibre>
  </div>
</Layout>
