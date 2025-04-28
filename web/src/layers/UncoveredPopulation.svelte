<script lang="ts">
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { DataZones } from "../types";
  import { uncoveredPopulation as show } from "./stores";

  let lastUpdate = 0;
  let data: DataZones = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.getDataZones();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }
</script>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("uncovered-population-fill", false)}
    filter={["!", ["get", "reachable"]]}
    manageHoverState
    paint={{
      "fill-color": "red",
      "fill-opacity": hoverStateFilter(0, 0.5),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    hoverCursor="pointer"
  >
    <Popup openOn="click" let:props>
      Zone {props.id} with a population of {props.population.toLocaleString()}
      is not reachable.
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("uncovered-population-outline")}
    filter={["!", ["get", "reachable"]]}
    paint={{
      "line-color": "red",
      "line-width": 1,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
