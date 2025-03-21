<script lang="ts">
  import "./style.css";
  import "@scottish-government/design-system/dist/css/design-system.css";
  import "@scottish-government/design-system/dist/scripts/design-system.js";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import * as Comlink from "comlink";
  import type { Map, StyleSpecification } from "maplibre-gl";
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
  import currentPoiIcon from "../assets/current_poi.png";
  import hospital1Icon from "../assets/hospital_reachable.png";
  import hospital2Icon from "../assets/hospital_unreachable.png";
  import school1Icon from "../assets/school_reachable.png";
  import school2Icon from "../assets/school_unreachable.png";
  import BulkEditMode from "./BulkEditMode.svelte";
  import { layerId } from "./common";
  import DisableInteractiveLayers from "./common/DisableInteractiveLayers.svelte";
  import { getKey } from "./common/files";
  import { controlsContents, Layout, mapContents } from "./common/layout";
  import StreetView from "./common/StreetView.svelte";
  import EditRouteMode from "./edit/EditRouteMode.svelte";
  import EvaluateJourneyMode from "./EvaluateJourneyMode.svelte";
  import ReferenceLayers from "./layers/ReferenceLayers.svelte";
  import BottomPanel from "./layers/roads/BottomPanel.svelte";
  import LegendPanel from "./layers/roads/LegendPanel.svelte";
  import RightLayers from "./layers/roads/RightLayers.svelte";
  import MainMode from "./MainMode.svelte";
  import {
    assetUrl,
    backend,
    boundaryName,
    currentFilename,
    currentStage,
    map as mapStore,
    maptilerApiKey,
    mode,
    mutationCounter,
    remoteStorage,
    routeA,
    routeB,
    stats,
    zoom,
  } from "./stores";
  import TopBar from "./TopBar.svelte";
  import type { Backend } from "./worker";
  import workerWrapper from "./worker?worker";

  let loading = "";
  let progress = 0;

  let map: Map;
  $: if (map) {
    map.keyboard.disableRotation();
    map.dragRotate.disable();
    map.touchZoomRotate.disableRotation();
    mapStore.set(map);
  }

  onMount(async () => {
    // @ts-expect-error This really exists for the SG design system, but TS doesn't know about it
    window.DS.initAll();

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
    let openFile = params.get("file");
    if (openFile) {
      let item = window.localStorage.getItem(getKey($boundaryName, openFile));
      if (item) {
        try {
          await backendWorker.loadSavefile(item);
          $currentFilename = openFile;
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

  let controlsDiv: HTMLDivElement;
  let mapDiv: HTMLDivElement;
  $: if (controlsDiv && $controlsContents) {
    controlsDiv.innerHTML = "";
    controlsDiv.appendChild($controlsContents);
  }
  $: if (mapDiv && $mapContents) {
    mapDiv.innerHTML = "";
    mapDiv.appendChild($mapContents);
  }

  async function getStyle(): Promise<StyleSpecification | string> {
    // streets-v2 uses a fill-extrusion layer for 3D buildings that's very distracting. Remove it, and make the regular buildings layer display at high zoom instead.
    let resp = await fetch(
      `https://api.maptiler.com/maps/streets-v2/style.json?key=${maptilerApiKey}`,
    );
    let json = await resp.json();
    json.layers = json.layers.filter((l: any) => l.id != "Building 3D");
    delete json.layers.find((l: any) => l.id == "Building")!.maxzoom;
    return json;
  }

  // Start less than $mutationCounter
  let lastUpdateFastStats = 0;
  async function recalcFastStats() {
    if ($backend && lastUpdateFastStats != $mutationCounter) {
      console.time("Recalculate fast stats");
      $stats = await $backend.recalculateStats();
      console.timeEnd("Recalculate fast stats");
      lastUpdateFastStats = $mutationCounter;
    }
  }
  $: if ($backend && $mutationCounter > 0) {
    recalcFastStats();
  }
</script>

<svelte:head>
  <title>Network Planning Workspace - {$boundaryName}</title>
</svelte:head>

<Loading {loading} {progress} />

<Layout>
  <header slot="top">
    <TopBar />
  </header>
  <main slot="controls">
    <div bind:this={controlsDiv} />
  </main>

  <main slot="map" class="map-container">
    {#await getStyle() then style}
      <MapLibre
        {style}
        maxZoom={19}
        bind:map
        bind:zoom={$zoom}
        on:error={(e) => {
          // @ts-expect-error ErrorEvent isn't exported
          console.log(e.detail.error);
        }}
        images={[
          { id: "schools_reachable", url: school1Icon },
          { id: "schools_unreachable", url: school2Icon },
          { id: "gp_hospitals_reachable", url: hospital1Icon },
          { id: "gp_hospitals_unreachable", url: hospital2Icon },
          { id: "current_poi", url: currentPoiIcon },
        ]}
        hash
      >
        <NavigationControl showCompass={false} />
        <ScaleControl />
        <Geocoder {map} apiKey={maptilerApiKey} country="gb" />
        <DisableInteractiveLayers />
        {#if $currentStage != "LocalAccess"}
          <StreetView />
        {/if}

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
          <RightLayers />
          <LegendPanel />
          <BottomPanel />

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
    {/await}
  </main>
</Layout>

<style>
  :global(body) {
    margin: 0;
  }

  .map-container {
    position: relative;
    width: 100%;
    height: 100%;
  }
</style>
