<script lang="ts">
  import centroid from "@turf/centroid";
  import type { Position } from "geojson";
  import PrevNext from "../common/PrevNext.svelte";
  import { backend, currentStage, map, mutationCounter } from "../stores";
  import type { PoiKind } from "../types";
  import { currentPOI } from "./stores";

  type Reachability = "reachable" | "unreachable" | "all";

  let filterReachability: Reachability = "unreachable";
  let filterKind: PoiKind | "all" = "all";

  let lastUpdate = 0;
  let lastFilter: [PoiKind | "all", Reachability] = [
    filterKind,
    filterReachability,
  ];
  // [kind, idx, name, reachable, position]
  let pois: {
    kind: PoiKind;
    idx: number;
    name: string;
    reachable: boolean;
    position: Position;
  }[] = [];
  // idx into this list, not of the POI itself
  let idx = 0;

  // TODO It'd be nice to sort these roughly by distance to the viewport?
  async function recalc(
    filterKind: PoiKind | "all",
    filterReachability: Reachability,
  ) {
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

    if (filterKind == "all" || filterKind == "schools") {
      for (let f of (await $backend.getSchools()).features) {
        if (
          filterReachability == "all" ||
          f.properties.reachable == (filterReachability == "reachable")
        ) {
          pois.push({
            kind: f.properties.poi_kind,
            idx: f.properties.idx,
            name: f.properties.name || "This school",
            reachable: f.properties.reachable,
            position: f.geometry.coordinates,
          });
        }
      }
    }

    if (filterKind == "all" || filterKind == "gp_hospitals") {
      for (let f of (await $backend.getGpHospitals()).features) {
        if (
          filterReachability == "all" ||
          f.properties.reachable == (filterReachability == "reachable")
        ) {
          pois.push({
            kind: f.properties.poi_kind,
            idx: f.properties.idx,
            name: f.properties.name || "This GP or hospital",
            reachable: f.properties.reachable,
            position: f.geometry.coordinates,
          });
        }
      }
    }

    if (filterKind == "all" || filterKind == "greenspaces") {
      for (let f of (await $backend.getGreenspaces()).features) {
        if (f.properties.kind == "access point") {
          continue;
        }
        if (
          filterReachability == "all" ||
          f.properties.reachable == (filterReachability == "reachable")
        ) {
          // TODO Slow to calculate this constantly
          pois.push({
            kind: f.properties.poi_kind,
            idx: f.properties.idx!,
            name: f.properties.name || "This greenspace",
            reachable: f.properties.reachable!,
            position: centroid(f).geometry.coordinates,
          });
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

  $: if (pois.length > 0) {
    $currentPOI = {
      idx: pois[idx].idx,
      kind: pois[idx].kind,
    };
  } else {
    $currentPOI = null;
  }

  function warp(idx: number) {
    if (idx == 0 || !$map) {
      return;
    }
    $map.flyTo({
      center: pois[idx].position as [number, number],
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
    <option value="schools">Schools</option>
    <option value="gp_hospitals">GPs/hospitals</option>
    <option value="greenspaces">Greenspaces</option>
  </select>
</label>

{#if pois.length > 0}
  {#if pois[idx].reachable}
    <p>
      {pois[idx].name} is connected to the network. The blue path shows the route
      through quiet streets to the network.
    </p>
  {:else}
    <p>
      {pois[idx].name} is not connected to the network, because there are red severances
      surronding it. Creating a local access route along the black path would fix
      this.
    </p>
  {/if}

  <PrevNext list={pois} bind:idx />
{/if}
