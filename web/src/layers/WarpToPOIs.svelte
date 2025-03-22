<script lang="ts">
  import { backend, currentStage, map, mutationCounter } from "../stores";
  import type { PoiKind } from "../types";
  import { currentPOI, type POI } from "./stores";

  let lastUpdate = 0;

  let allPOIs: POI[] = [];

  let filterKind: PoiKind | "all" = "all";
  let filterIdx = 0;
  let filteredPOIs: POI[] = [];

  async function recalc() {
    if (!$backend || lastUpdate == $mutationCounter) {
      return;
    }

    let list: [POI, number][] = [];

    for (let f of (await $backend.getPOIs()).features) {
      list.push([
        {
          kind: f.properties.poi_kind,
          idx: f.properties.idx,
          description: f.properties.description,
          reachable: f.properties.reachable,
          pt: f.geometry.coordinates as [number, number],
        },
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
        f.properties.sort!,
      ]);
    }

    // The sort key uses a Hilbert curve to group nearby POIs together.
    list.sort((a, b) => a[1] - b[1]);

    allPOIs = list.map(([poi, _]) => poi);
    lastUpdate = $mutationCounter;

    refilterPOIs();
  }
  $: if ($currentStage == "LocalAccess" && $mutationCounter > 0) {
    recalc();
  }

  function refilterPOIs() {
    filterIdx = 0;
    let unconnected = allPOIs.filter((poi) => !poi.reachable);
    filteredPOIs =
      filterKind == "all"
        ? unconnected
        : unconnected.filter((poi) => poi.kind == filterKind);
  }

  function warp() {
    if ($map && $currentPOI) {
      $map.flyTo({
        center: $currentPOI.pt,
        zoom: 14,
      });
    }
  }

  function prev() {
    if (filterIdx != 0) {
      filterIdx--;
      $currentPOI = filteredPOIs[filterIdx];
      warp();
    }
  }

  function next() {
    if (filterIdx != filteredPOIs.length - 1) {
      filterIdx++;
      $currentPOI = filteredPOIs[filterIdx];
      warp();
    }
  }
</script>

<h4>Fix unconnected POIs</h4>

<div>
  <label>
    Filter POIs:
    <select bind:value={filterKind} on:change={refilterPOIs}>
      <option value="all">All</option>
      <option value="schools">Schools</option>
      <option value="gp_hospitals">GPs/hospitals</option>
      <option value="greenspaces">Greenspaces</option>
    </select>
  </label>
</div>

{#if filteredPOIs.length > 0}
  <div
    style="display: flex; justify-content: space-between; align-items: center;"
  >
    <button disabled={filterIdx == 0} on:click={prev}>Previous</button>
    {filterIdx + 1} / {filteredPOIs.length}
    <button disabled={filterIdx == filteredPOIs.length - 1} on:click={next}>
      Next
    </button>
  </div>
{/if}
