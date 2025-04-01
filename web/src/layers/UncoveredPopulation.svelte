<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
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

<GeoJSON {data}>
  <LineLayer
    {...layerId("uncovered-population")}
    filter={["!", ["get", "reachable"]]}
    paint={{
      "line-color": "red",
      "line-width": 3,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
