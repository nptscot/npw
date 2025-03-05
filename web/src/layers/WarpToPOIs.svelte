<script lang="ts">
  import centroid from "@turf/centroid";
  import type { Position } from "geojson";
  import PrevNext from "../common/PrevNext.svelte";
  import { backend, currentStage, map, mutationCounter } from "../stores";

  let filterKind: "school" | "gp" | "greenspace" | "all" = "all";

  let lastUpdate = 0;
  let lastFilter = filterKind;
  let unreachablePOIs: [string, Position][] = [];
  let idx = 0;

  // TODO It'd be nice to sort these roughly by distance to the viewport?
  async function recalc(filterKind: "school" | "gp" | "greenspace" | "all") {
    if (
      !$backend ||
      (lastUpdate == $mutationCounter && lastFilter == filterKind)
    ) {
      return;
    }

    idx = 0;
    unreachablePOIs = [];

    if (filterKind == "all" || filterKind == "school") {
      for (let f of (await $backend.getSchools()).features) {
        if (!f.properties.reachable) {
          unreachablePOIs.push([
            f.properties.name || "This school",
            f.geometry.coordinates,
          ]);
        }
      }
    }

    if (filterKind == "all" || filterKind == "gp") {
      for (let f of (await $backend.getGpHospitals()).features) {
        if (!f.properties.reachable) {
          unreachablePOIs.push([
            f.properties.name || "This GP or hospital",
            f.geometry.coordinates,
          ]);
        }
      }
    }

    if (filterKind == "all" || filterKind == "greenspace") {
      for (let f of (await $backend.getGreenspaces()).features) {
        if (f.properties.kind != "access point" && !f.properties.reachable) {
          // TODO Slow to calculate this constantly
          unreachablePOIs.push([
            f.properties.name || "This greenspace",
            centroid(f).geometry.coordinates,
          ]);
        }
      }
    }

    unreachablePOIs = unreachablePOIs;
    lastUpdate = $mutationCounter;
    lastFilter = filterKind;
  }

  $: if ($currentStage == "LocalAccess" && $mutationCounter > 0) {
    recalc(filterKind);
  }

  // TODO Never warp for 0, even if we go back to it
  $: warp(idx);

  function warp(idx: number) {
    if (idx == 0 || !$map) {
      return;
    }
    $map.flyTo({
      center: unreachablePOIs[idx][1] as [number, number],
      zoom: 14,
    });
  }
</script>

<label>
  Filter POIs:
  <select bind:value={filterKind}>
    <option value="all">All</option>
    <option value="school">Schools</option>
    <option value="gp">GPs/hospitals</option>
    <option value="greenspace">Greenspaces</option>
  </select>
</label>

{#if unreachablePOIs.length > 0}
  <b>Fix next unreachable POI</b>
  <p>{unreachablePOIs[idx][0]}</p>
  <PrevNext list={unreachablePOIs} bind:idx />
{/if}
