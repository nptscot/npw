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
</script>

<h4>Find unconnected POIs</h4>

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

<br />

<button
  class="ds_button ds_button--secondary"
  disabled={filteredPOIs.length == 0}
  on:click={findAnother}
>
  Find another
</button>
