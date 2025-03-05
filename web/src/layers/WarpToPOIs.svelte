<script lang="ts">
  import centroid from "@turf/centroid";
  import type { Position } from "geojson";
  import PrevNext from "../common/PrevNext.svelte";
  import { backend, currentStage, map, mutationCounter } from "../stores";

  type Reachability = "reachable" | "unreachable" | "all";
  type Kind = "school" | "gp" | "greenspace" | "all";

  let filterReachability: Reachability = "unreachable";
  let filterKind: Kind = "all";

  let lastUpdate = 0;
  let lastFilter: [Kind, Reachability] = [filterKind, filterReachability];
  // [name, reachable, position]
  let pois: [string, boolean, Position][] = [];
  let idx = 0;

  // TODO It'd be nice to sort these roughly by distance to the viewport?
  async function recalc(filterKind: Kind, filterReachability: Reachability) {
    if (
      !$backend ||
      (lastUpdate == $mutationCounter &&
        lastFilter[0] == filterKind &&
        lastFilter[1] == filterReachability)
    ) {
      return;
    }

    idx = 0;
    pois = [];

    if (filterKind == "all" || filterKind == "school") {
      for (let f of (await $backend.getSchools()).features) {
        if (
          filterReachability == "all" ||
          f.properties.reachable == (filterReachability == "reachable")
        ) {
          pois.push([
            f.properties.name || "This school",
            f.properties.reachable,
            f.geometry.coordinates,
          ]);
        }
      }
    }

    if (filterKind == "all" || filterKind == "gp") {
      for (let f of (await $backend.getGpHospitals()).features) {
        if (
          filterReachability == "all" ||
          f.properties.reachable == (filterReachability == "reachable")
        ) {
          pois.push([
            f.properties.name || "This GP or hospital",
            f.properties.reachable,
            f.geometry.coordinates,
          ]);
        }
      }
    }

    if (filterKind == "all" || filterKind == "greenspace") {
      for (let f of (await $backend.getGreenspaces()).features) {
        if (f.properties.kind == "access point") {
          continue;
        }
        if (
          filterReachability == "all" ||
          f.properties.reachable == (filterReachability == "reachable")
        ) {
          // TODO Slow to calculate this constantly
          pois.push([
            f.properties.name || "This greenspace",
            f.properties.reachable!,
            centroid(f).geometry.coordinates,
          ]);
        }
      }
    }

    pois = pois;
    lastUpdate = $mutationCounter;
    lastFilter = [filterKind, filterReachability];
  }

  $: if ($currentStage == "LocalAccess" && $mutationCounter > 0) {
    recalc(filterKind, filterReachability);
  }

  // TODO Never warp for 0, even if we go back to it
  $: warp(idx);

  function warp(idx: number) {
    if (idx == 0 || !$map) {
      return;
    }
    $map.flyTo({
      center: pois[idx][2] as [number, number],
      zoom: 14,
    });
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
    <option value="school">Schools</option>
    <option value="gp">GPs/hospitals</option>
    <option value="greenspace">Greenspaces</option>
  </select>
</label>

{#if pois.length > 0}
  {#if pois[idx][1]}
    <p>
      {pois[idx][0]} is connected to the network. The blue path shows the route through
      quiet streets to the network.
    </p>
  {:else}
    <p>
      {pois[idx][0]} is not connected to the network, because there are red severances
      surronding it. Creating a local access route along the black path would fix
      this.
    </p>
  {/if}

  <PrevNext list={pois} bind:idx />
{/if}
