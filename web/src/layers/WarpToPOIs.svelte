<script lang="ts">
  import centroid from "@turf/centroid";
  import type { Position } from "geojson";
  import PrevNext from "../common/PrevNext.svelte";
  import {
    autosave,
    backend,
    currentStage,
    map,
    mutationCounter,
  } from "../stores";
  import type { PoiKind } from "../types";
  import { currentPOI, type CurrentPOI } from "./stores";

  type Reachability = "reachable" | "unreachable" | "all";
  interface POI {
    poi_kind: PoiKind;
    idx: number;
    name: string;
    reachable: boolean;
    position: Position;
  }

  let filterReachability: Reachability = "unreachable";
  let filterKind: PoiKind | "all" = "all";

  let lastUpdate = 0;

  let allPOIs: POI[] = [];

  $: filteredPOIs = filterPOIs(allPOIs, filterKind, filterReachability);
  let filterIdx = 0;

  async function recalc() {
    if (!$backend || lastUpdate == $mutationCounter) {
      return;
    }

    allPOIs = [];

    for (let f of (await $backend.getSchools()).features) {
      let name = `${f.properties.name || "This school"} (a ${f.properties.kind} school with ${f.properties.pupils} pupils)`;
      allPOIs.push({
        poi_kind: f.properties.poi_kind,
        idx: f.properties.idx,
        name,
        reachable: f.properties.reachable,
        position: f.geometry.coordinates,
      });
    }

    for (let f of (await $backend.getGpHospitals()).features) {
      allPOIs.push({
        poi_kind: f.properties.poi_kind,
        idx: f.properties.idx,
        name: f.properties.name || `This ${f.properties.kind}`,
        reachable: f.properties.reachable,
        position: f.geometry.coordinates,
      });
    }

    for (let f of (await $backend.getGreenspaces()).features) {
      if (f.properties.kind == "access point") {
        continue;
      }
      // TODO Slow to calculate this constantly
      allPOIs.push({
        poi_kind: f.properties.poi_kind,
        idx: f.properties.idx!,
        name: f.properties.name || "This greenspace",
        reachable: f.properties.reachable!,
        position: centroid(f).geometry.coordinates,
      });
    }

    allPOIs = allPOIs;
    lastUpdate = $mutationCounter;
  }
  $: if ($currentStage == "LocalAccess" && $mutationCounter > 0) {
    recalc();
  }

  // TODO It'd be nice to sort these roughly by distance to the viewport?
  function filterPOIs(
    allPOIs: POI[],
    filterKind: PoiKind | "all",
    filterReachability: Reachability,
  ): POI[] {
    filterIdx = 0;

    return allPOIs.filter((poi) => {
      if (filterKind != "all" && poi.poi_kind != filterKind) {
        return false;
      }
      if (
        filterReachability != "all" &&
        poi.reachable != (filterReachability == "reachable")
      ) {
        return false;
      }
      return true;
    });
  }

  $: warp($currentPOI);

  $: if (filteredPOIs.length > 0) {
    $currentPOI = {
      idx: filteredPOIs[filterIdx].idx,
      kind: filteredPOIs[filterIdx].poi_kind,
      reachable: filteredPOIs[filterIdx].reachable,
    };
  } else {
    $currentPOI = null;
  }

  // When currentPOI changes elsewhere from clicking on the map, make the filtered list work
  function resetFilters(currentPOI: CurrentPOI | null) {
    if (!currentPOI) {
      return;
    }

    for (let [i, poi] of filteredPOIs.entries()) {
      if (poi.poi_kind == currentPOI.kind && poi.idx == currentPOI.idx) {
        filterIdx = i;
        return;
      }
    }

    // Relax the filters
    filterReachability = "all";
    filterKind = "all";
    // Do this immediately
    filteredPOIs = filterPOIs(allPOIs, filterKind, filterReachability);

    // Then repeat the above
    for (let [i, poi] of filteredPOIs.entries()) {
      if (poi.poi_kind == currentPOI.kind && poi.idx == currentPOI.idx) {
        filterIdx = i;
        return;
      }
    }
    console.error(`Clicked on ${currentPOI} but can't find it!`);
  }
  $: resetFilters($currentPOI);

  function warp(currentPOI: CurrentPOI | null) {
    if (!$map || !currentPOI) {
      return;
    }
    let poi = allPOIs.find(
      (poi) => poi.poi_kind == currentPOI.kind && poi.idx == currentPOI.idx,
    );
    if (poi) {
      $map.flyTo({
        center: poi.position as [number, number],
        zoom: 14,
      });
    }
  }

  async function fixUnreachable() {
    if ($currentPOI) {
      let input = await $backend!.fixUnreachablePOI(
        $currentPOI.kind,
        $currentPOI.idx,
      );
      await $backend!.setRoute(null, input);
      await autosave();
    }
  }
</script>

<label>
  Show reachable POIs:
  <select bind:value={filterReachability}>
    <option value="all">All</option>
    <option value="reachable">Only reachable POIs</option>
    <option value="unreachable">Only unreachable POIs</option>
  </select>
</label>

<label>
  Filter POIs:
  <select bind:value={filterKind}>
    <option value="all">All</option>
    <option value="schools">Schools</option>
    <option value="gp_hospitals">GPs/hospitals</option>
    <option value="greenspaces">Greenspaces</option>
  </select>
</label>

{#if filteredPOIs.length > 0}
  {#if filteredPOIs[filterIdx].reachable}
    <p>
      {filteredPOIs[filterIdx].name} is connected to the network. The blue path shows
      the route through quiet streets to the network.
    </p>
  {:else}
    <p>
      {filteredPOIs[filterIdx].name} is not connected to the network, because there
      are red severances surronding it.
    </p>

    <button on:click={fixUnreachable}>
      Add the black local access route to fix
    </button>
  {/if}

  <PrevNext list={filteredPOIs} bind:idx={filterIdx} />
{/if}
