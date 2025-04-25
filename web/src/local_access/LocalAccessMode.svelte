<script lang="ts">
  import { MapEvents } from "svelte-maplibre";
  import { tierColors } from "../colors";
  import { BackLink, prettyPrintDistance } from "../common";
  import { SplitComponent } from "../common/layout";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import {
    autosave,
    backend,
    currentStage,
    exitCurrentStage,
    map,
    mode,
    mutationCounter,
    zoom,
  } from "../stores";
  import DebugReachability from "./DebugReachability.svelte";
  import Greenspaces from "./Greenspaces.svelte";
  import PointPOIs from "./PointPOIs.svelte";
  import {
    currentPOI,
    debugReachabilityCurrentPOI,
    filterKind,
    fixCurrentPOI,
    type POI,
  } from "./stores";
  import StreetViewPOI from "./StreetViewPOI.svelte";

  let lastUpdate = 0;

  let allPOIs: POI[] = [];

  let filterIdx = 0;
  let filteredPOIs: POI[] = [];

  async function recalc() {
    if (!$backend || lastUpdate == $mutationCounter) {
      return;
    }

    let list: [POI, number, number][] = [];

    for (let f of (await $backend.getPOIs()).features) {
      list.push([
        {
          kind: f.properties.poi_kind,
          idx: f.properties.idx,
          description: f.properties.description,
          reachable: f.properties.reachable,
          pt: f.geometry.coordinates as [number, number],
        },
        f.properties.poi_kind == "schools" ? 1 : 2,
        f.properties.sort,
      ]);
    }

    for (let f of (await $backend.getGreenspaces()).features) {
      if (f.properties.kind == "access point") {
        continue;
      }
      list.push([
        {
          kind: f.properties.poi_kind,
          idx: f.properties.idx!,
          description: f.properties.description!,
          reachable: f.properties.reachable!,
          pt: f.properties.centroid!,
        },
        3,
        f.properties.sort!,
      ]);
    }

    // Prioritize schools, then GPs, then greenspaces. Within each group, use a
    // Hilbert curve to group nearby POIs together.
    list.sort((a, b) => {
      if (a[1] != b[1]) {
        return a[1] - b[1];
      }
      return a[2] - b[2];
    });

    allPOIs = list.map(([poi, _a, _b]) => poi);
    lastUpdate = $mutationCounter;

    refilterPOIs();

    // While a POI is focused, data changed. Re-fetch it; maybe its reachability has changed.
    if ($currentPOI) {
      let kind = $currentPOI.kind;
      let idx = $currentPOI.idx;
      $currentPOI =
        allPOIs.find((poi) => poi.kind == kind && poi.idx == idx) || null;
    }
  }
  $: if ($currentStage == "LocalAccess" && $mutationCounter > 0) {
    recalc();
  }

  function refilterPOIs() {
    filterIdx = 0;
    let unconnected = allPOIs.filter((poi) => !poi.reachable);
    filteredPOIs =
      $filterKind == "all"
        ? unconnected
        : unconnected.filter((poi) => poi.kind == $filterKind);
  }

  function warp() {
    if ($map && $currentPOI) {
      $map.easeTo({
        center: $currentPOI.pt,
      });
    }
  }

  function findAnother() {
    if (filterIdx != filteredPOIs.length - 1) {
      filterIdx++;
    } else {
      filterIdx = 0;
    }

    $currentPOI = filteredPOIs[filterIdx];
    warp();
  }

  async function fixUnreachable() {
    if ($currentPOI) {
      let input = await $backend!.fixUnreachablePOI(
        $currentPOI.kind,
        $currentPOI.idx,
      );
      await $backend!.setRoute(null, input.properties);
      await autosave();

      // Let recalc happen to see if the fix succeeded
    }
  }

  async function onKeyDown(e: KeyboardEvent) {
    if (e.key == "a" && $currentPOI && !$currentPOI.reachable) {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT") {
        e.preventDefault();
        await fixUnreachable();
      }
    }

    if (e.key == "n" && $currentPOI && filteredPOIs.length > 0) {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT") {
        e.preventDefault();
        findAnother();
      }
    }
  }

  function startFixing() {
    if ($zoom && $zoom < 14 && $map) {
      $map.setZoom(14);
    }
    findAnother();
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="controls" class="left">
    <div class="main-controls">
      {#if !$currentPOI}
        <header
          class="ds_page-header"
          style="display: flex; justify-content: space-between;"
        >
          <h2 class="ds_page-header__title">Design local access network</h2>
        </header>

        <BackLink
          on:click={() => {
            exitCurrentStage();
            $mode = { kind: "overview" };
          }}
        >
          Back to project overview
        </BackLink>

        <p>
          Your network needs to provide connectivity to key points of interest,
          such as schools, hospitals and green spaces. POIs with severed
          connectivity are shown in red and need to be fixed.
        </p>

        <div class="ds_select-wrapper ds_input--fluid-two-thirds">
          <select
            class="ds_select"
            bind:value={$filterKind}
            on:change={refilterPOIs}
          >
            <option value="all">Showing all POIs</option>
            <option value="schools">Schools</option>
            <option value="gp_hospitals">GPs/hospitals</option>
            <option value="greenspaces">Greenspaces</option>
          </select>
          <span class="ds_select-arrow" aria-hidden="true"></span>
        </div>

        <div>
          <button
            class="ds_button"
            on:click={startFixing}
            disabled={filteredPOIs.length == 0}
          >
            Fix connectivity for remaining POIs
          </button>
        </div>
      {:else if $currentPOI}
        <header class="ds_page-header">
          <span
            class="ds_page-header__label ds_content-label"
            style:color={tierColors.LocalAccess}
          >
            Local access
          </span>
          <h2 class="ds_page-header__title">Fix connectivity for a POI</h2>
        </header>

        <BackLink on:click={() => ($currentPOI = null)}>
          Back to local access overview
        </BackLink>

        {#if $currentPOI.reachable}
          <p>
            {$currentPOI.description} is connected to the network.
          </p>
          {#if $debugReachabilityCurrentPOI}
            <p>
              The dashed blue path of length {prettyPrintDistance(
                $debugReachabilityCurrentPOI.length_meters,
              )} shows the route through quiet streets to the network.
            </p>
          {/if}
        {:else}
          <p>
            {$currentPOI.description} is not connected to the network.
          </p>
          {#if $fixCurrentPOI}
            <p>
              A suggested local access route of length {prettyPrintDistance(
                $fixCurrentPOI.properties.length_meters,
              )} is shown dashed.
            </p>
          {/if}

          <div>
            <button class="ds_button" on:click={fixUnreachable}>
              Add the dashed line to fix (
              <kbd>a</kbd>
              )
            </button>
          </div>

          <div>
            <button
              class="ds_button ds_button--secondary"
              on:click={() => ($mode = { kind: "edit-route", id: null })}
            >
              Or draw a new route line manually
            </button>
          </div>
        {/if}

        <button
          class="ds_button ds_button--secondary"
          disabled={filteredPOIs.length == 0}
          on:click={findAnother}
          style:float="right"
        >
          Next POI (
          <kbd>n</kbd>
          )
        </button>
      {/if}
    </div>

    <LeftSidebarStats />
  </div>

  <div slot="map">
    <MapEvents on:click={() => ($currentPOI = null)} />

    <Greenspaces />
    <PointPOIs />
    <DebugReachability current={$currentPOI} />

    <StreetViewPOI />
  </div>
</SplitComponent>

<style>
  .left {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
