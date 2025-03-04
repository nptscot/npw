<script lang="ts">
  import centroid from "@turf/centroid";
  import type { Position } from "geojson";
  import PrevNext from "../common/PrevNext.svelte";
  import { backend, currentStage, map, mutationCounter } from "../stores";

  let lastUpdate = 0;
  let unreachablePOIs: [string, Position][] = [];
  let idx = 0;

  // TODO It'd be nice to sort these roughly by distance to the viewport?
  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      idx = 0;
      unreachablePOIs = [];
      for (let f of (await $backend.getSchools()).features) {
        if (!f.properties.reachable) {
          unreachablePOIs.push([
            f.properties.name || "This school",
            f.geometry.coordinates,
          ]);
        }
      }
      for (let f of (await $backend.getGpHospitals()).features) {
        if (!f.properties.reachable) {
          unreachablePOIs.push([
            f.properties.name || "This GP or hospital",
            f.geometry.coordinates,
          ]);
        }
      }
      for (let f of (await $backend.getGreenspaces()).features) {
        if (f.properties.kind != "access point" && !f.properties.reachable) {
          // TODO Slow to calculate this constantly
          unreachablePOIs.push([
            f.properties.name || "This greenspace",
            centroid(f).geometry.coordinates,
          ]);
        }
      }

      unreachablePOIs = unreachablePOIs;
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($currentStage == "LocalAccess" && $mutationCounter > 0) {
    recalc();
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

{#if unreachablePOIs.length > 0}
  <b>Fix next unreachable POI</b>
  <p>{unreachablePOIs[idx][0]}</p>
  <PrevNext list={unreachablePOIs} bind:idx />
{/if}
