<script lang="ts">
  import "../style.css";
  import "@scottish-government/design-system/dist/css/design-system.css";
  import "@scottish-government/design-system/dist/scripts/design-system.js";
  import "@fortawesome/fontawesome-free/css/all.min.css";
  import * as Comlink from "comlink";
  import type { Map, StyleSpecification } from "maplibre-gl";
  import { onMount } from "svelte";
  import {
    FillLayer,
    GeoJSON,
    LineLayer,
    MapLibre,
    NavigationControl,
    ScaleControl,
  } from "svelte-maplibre";
  import { fetchWithProgress } from "svelte-utils";
  import favicon from "../../assets/favicon.ico?url";
  import hospital3Icon from "../../assets/hospital_ignore.png";
  import hospital1Icon from "../../assets/hospital_reachable.png";
  import hospital2Icon from "../../assets/hospital_unreachable.png";
  import railwayStation3Icon from "../../assets/railway_station_ignore.png";
  import railwayStation1Icon from "../../assets/railway_station_reachable.png";
  import railwayStation2Icon from "../../assets/railway_station_unreachable.png";
  import school3Icon from "../../assets/school_ignore.png";
  import school1Icon from "../../assets/school_reachable.png";
  import school2Icon from "../../assets/school_unreachable.png";
  import AssessMode from "../assess/AssessMode.svelte";
  import { backfillSavefile } from "../backfill";
  import BulkEditMode from "../BulkEditMode.svelte";
  import {
    countryBounds,
    layerId,
    LoadingSpinner,
    stripPrefix,
  } from "../common";
  import DisableInteractiveLayers from "../common/DisableInteractiveLayers.svelte";
  import { getKey } from "../common/files";
  import Geocoder from "../common/Geocoder.svelte";
  import { controlsContents, Layout, mapContents } from "../common/layout";
  import Loading from "../common/Loading.svelte";
  import ReportProblemMap from "../common/ReportProblemMap.svelte";
  import ReportProblemModal from "../common/ReportProblemModal.svelte";
  import SettlementPicker from "../common/SettlementPicker.svelte";
  import StreetView from "../common/StreetView.svelte";
  import SwitchBasemap from "../common/SwitchBasemap.svelte";
  import EditRouteMode from "../edit/EditRouteMode.svelte";
  import ReviewSectionsMode from "../edit/ReviewSectionsMode.svelte";
  import EvaluateJourneyMode from "../EvaluateJourneyMode.svelte";
  import ReferenceLayers from "../layers/ReferenceLayers.svelte";
  import BottomPanel from "../layers/roads/BottomPanel.svelte";
  import LegendPanel from "../layers/roads/LegendPanel.svelte";
  import RightLayers from "../layers/roads/RightLayers.svelte";
  import LocalAccessMode from "../local_access/LocalAccessMode.svelte";
  import MainMode from "../MainMode.svelte";
  import OverviewMode from "../OverviewMode.svelte";
  import SetupMode from "../SetupMode.svelte";
  import {
    assetUrl,
    backend,
    basemap,
    boundaryName,
    country,
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
    type Mode,
  } from "../stores";
  import TopBar from "../TopBar.svelte";
  import type { Tier } from "../types";
  import type { InnerBackend } from "../worker";
  import { Backend } from "../worker_wrapper";
  import workerWrapper from "../worker?worker";

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
    loading = `Opening ${stripPrefix($boundaryName, "LAD_")}`;

    interface WorkerConstructor {
      new (): InnerBackend;
    }

    let MyWorker: Comlink.Remote<WorkerConstructor> = Comlink.wrap(
      new workerWrapper(),
    );
    let backendWorker = await new MyWorker();

    // Detect if we're running locally first
    let bytes: Uint8Array<ArrayBufferLike> = new Uint8Array();
    try {
      bytes = await fetchWithProgress(
        `${$country}/areas/${$boundaryName}.bin.gz`,
        (p) => {
          progress = p;
        },
      );
      console.log(`Using locally hosted files`);
      $remoteStorage = false;
    } catch (err) {
      console.log(`Using remote hosted files`);
      try {
        bytes = await fetchWithProgress(
          assetUrl(`areas/${$boundaryName}.bin.gz`),
          (p) => {
            progress = p;
          },
        );
      } catch (err) {
        window.alert(
          `Your browser proxy is blocking access to https://npw.scot. Error: ${err}`,
        );
      }
    }

    try {
      await backendWorker.loadFile(new Uint8Array(bytes));
    } catch (err) {
      window.alert(`Couldn't load: ${err}`);
    }

    let wrappedBackend = new Backend(backendWorker);

    // Load saved state?
    let openFile = params.get("file");
    if (openFile) {
      let item = window.localStorage.getItem(getKey($boundaryName, openFile));
      if (item) {
        try {
          let fixed = await backfillSavefile(item, $boundaryName);
          await wrappedBackend.loadSavefile(fixed);
          $currentFilename = openFile;
          $mode = { kind: "overview" };
        } catch (err) {
          window.alert(`Couldn't open project ${openFile}. Error: ${err}`);
          // Continue with the initial setup mode
        }
      } else {
        window.alert(`No saved file called ${openFile}`);
      }
    }

    loading = "";

    let bbox = await wrappedBackend.getBounds();
    $routeA = {
      lng: lerp(0.4, bbox[0], bbox[2]),
      lat: lerp(0.4, bbox[1], bbox[3]),
    };
    $routeB = {
      lng: lerp(0.6, bbox[0], bbox[2]),
      lat: lerp(0.6, bbox[1], bbox[3]),
    };

    backend.set(wrappedBackend);
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

  let style: string | StyleSpecification | null = null;
  $: updateStyle($basemap);
  async function updateStyle(basemap: string) {
    if (basemap == "light") {
      let resp = await fetch(
        `https://api.maptiler.com/maps/dataviz-light/style.json?key=${maptilerApiKey}`,
      );
      let json = await resp.json();
      style = json;
      return;
    }

    // streets-v2 uses a fill-extrusion layer for 3D buildings that's very distracting. Remove it, and make the regular buildings layer display at high zoom instead.
    let resp = await fetch(
      `https://api.maptiler.com/maps/streets-v2/style.json?key=${maptilerApiKey}`,
    );
    let json = await resp.json();
    json.layers = json.layers.filter((l: any) => l.id != "Building 3D");
    delete json.layers.find((l: any) => l.id == "Building")!.maxzoom;
    style = json;
  }

  // Start less than $mutationCounter
  let lastUpdateFastStats = 0;
  async function recalcFastStats() {
    if ($backend && lastUpdateFastStats != $mutationCounter) {
      $stats = await $backend.recalculateStats();
      lastUpdateFastStats = $mutationCounter;
    }
  }
  $: if ($backend && $mutationCounter > 0) {
    recalcFastStats();
  }

  function invertLayer(mode: Mode, currentStage: Tier | "assessment"): string {
    if (mode.kind == "setup" || mode.kind == "overview") {
      return "all";
    }

    if (currentStage == "Primary" || currentStage == "Secondary") {
      return "inside";
    }
    if (currentStage == "LongDistance") {
      return "outside";
    }

    return "all";
  }
</script>

<svelte:head>
  <title>Network Planning Workspace - {$boundaryName}</title>
  <link rel="icon" type="image/x-icon" href={favicon} />
</svelte:head>

<Loading {loading} {progress}>
  <p style="font-size: 0.5em">
    If the progress bar does not change after a few seconds, then either your
    internet connection is slow or your browser proxy is blocking access to
    https://npw.scot. Try <a href="manual.html#diagnosing-problems-using-npw">
      these steps to diagnose problems
    </a>
    .
  </p>
</Loading>

<Layout>
  <header slot="top">
    <TopBar />
  </header>
  <main slot="controls">
    {#if $country != "scotland"}
      <b>Warning: This is an experimental version</b>
    {/if}
    <div class="controlsDiv" bind:this={controlsDiv} />
    {#if $backend}
      <ReportProblemModal />
    {/if}
  </main>

  <main slot="map" class="map-container">
    {#if style}
      <MapLibre
        {style}
        maxZoom={19}
        bounds={countryBounds[$country]}
        bind:map
        bind:zoom={$zoom}
        on:error={(e) => {
          // @ts-expect-error ErrorEvent isn't exported
          console.log(e.detail.error);
        }}
        images={[
          { id: "railway_stations_reachable", url: railwayStation1Icon },
          { id: "railway_stations_unreachable", url: railwayStation2Icon },
          { id: "railway_stations_ignore", url: railwayStation3Icon },
          { id: "schools_reachable", url: school1Icon },
          { id: "schools_unreachable", url: school2Icon },
          { id: "schools_ignore", url: school3Icon },
          { id: "gp_hospitals_reachable", url: hospital1Icon },
          { id: "gp_hospitals_unreachable", url: hospital2Icon },
          { id: "gp_hospitals_ignore", url: hospital3Icon },
        ]}
        hash
      >
        <NavigationControl showCompass={false} />
        <ScaleControl />
        <Geocoder {map} apiKey={maptilerApiKey} country="gb" />
        <LoadingSpinner />
        <SettlementPicker />
        <DisableInteractiveLayers />
        {#if $currentStage != "LocalAccess"}
          <StreetView />
        {/if}
        <ReportProblemMap />
        <SwitchBasemap />

        <div bind:this={mapDiv} />

        {#if $backend}
          {#await $backend.getInvertedBoundaryInsideSettlements() then data}
            <GeoJSON {data}>
              <FillLayer
                {...layerId("fade-study-area-inside")}
                paint={{ "fill-color": "black", "fill-opacity": 0.3 }}
                layout={{
                  visibility:
                    invertLayer($mode, $currentStage) == "inside"
                      ? "visible"
                      : "none",
                }}
              />
            </GeoJSON>
          {/await}
          {#await $backend.getInvertedBoundaryOutsideSettlements() then data}
            <GeoJSON {data}>
              <FillLayer
                {...layerId("fade-study-area-outside")}
                paint={{ "fill-color": "black", "fill-opacity": 0.3 }}
                layout={{
                  visibility:
                    invertLayer($mode, $currentStage) == "outside"
                      ? "visible"
                      : "none",
                }}
              />
            </GeoJSON>
          {/await}
          {#await $backend.getInvertedBoundaryForStudyArea() then data}
            <GeoJSON {data}>
              <FillLayer
                {...layerId("fade-study-area-entire")}
                paint={{ "fill-color": "black", "fill-opacity": 0.3 }}
                layout={{
                  visibility:
                    invertLayer($mode, $currentStage) == "all"
                      ? "visible"
                      : "none",
                }}
              />
            </GeoJSON>
          {/await}
          {#await $backend.getStudyAreaBoundary() then data}
            <GeoJSON {data}>
              <LineLayer
                {...layerId("study-area-outline")}
                paint={{
                  "line-color": "black",
                  "line-width": 2,
                  "line-dasharray": [2, 1],
                }}
              />
            </GeoJSON>
          {/await}

          <ReferenceLayers />
          <RightLayers />
          <LegendPanel />
          {#if $mode.kind != "setup" || $mode.subpage == "new-project"}
            <BottomPanel />
          {/if}

          {#if $mode.kind == "setup"}
            <SetupMode bind:subpage={$mode.subpage} />
          {:else if $mode.kind == "overview"}
            <OverviewMode />
          {:else if $mode.kind == "main"}
            {#if $currentStage == "assessment"}
              <AssessMode />
            {:else if $currentStage == "LocalAccess"}
              <LocalAccessMode />
            {:else}
              <MainMode />
            {/if}
          {:else if $mode.kind == "edit-route" && map}
            <EditRouteMode
              id={$mode.id}
              {map}
              bind:anyEdits={$mode.anyEdits}
              restoreWaypoints={$mode.restoreWaypoints}
            />
          {:else if $mode.kind == "review-sections"}
            <ReviewSectionsMode
              ids={$mode.ids}
              sections={$mode.sections}
              restoreWaypoints={$mode.restoreWaypoints}
            />
          {:else if $mode.kind == "evaluate-journey"}
            <EvaluateJourneyMode browse={$mode.browse} />
          {:else if $mode.kind == "bulk-edit"}
            <BulkEditMode />
          {/if}
        {/if}
      </MapLibre>
    {/if}
  </main>
</Layout>

<style>
  :global(body) {
    margin: 0;
  }

  main[slot="controls"] {
    height: 100%;
  }

  .controlsDiv {
    height: 100%;
  }

  .map-container {
    position: relative;
    width: 100%;
    height: 100%;
  }
</style>
