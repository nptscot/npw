<script lang="ts">
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { makeRamp, Popup } from "svelte-utils/map";
  import { deprived } from "../colors";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { DataZones } from "../types";
  import { populationStyle } from "./stores";

  let lastUpdate = 0;
  let data: DataZones = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      let gj = await $backend.getDataZones();
      // Filter for the top quintile only
      gj.features = gj.features.filter(
        (f) => f.properties.imd_percentile <= 20,
      );
      data = gj;
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($populationStyle == "deprived" && $mutationCounter > 0) {
    recalc();
  }
</script>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("simd")}
    manageHoverState
    paint={{
      "fill-color": makeRamp(
        ["get", "imd_percentile"],
        deprived.limits,
        deprived.colorScale,
      ),
      "fill-opacity": hoverStateFilter(0.7, 0.9),
    }}
    layout={{
      visibility: $populationStyle == "deprived" ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      <p>
        Data zone {props.id}
        {props.reachable ? "is" : "is not"} reachable. It has {props.population.toLocaleString()}
        people, and a SIMD rank of {props.imd_rank}, putting it in the {props.imd_percentile}
        percentile.
      </p>
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("simd-outline")}
    filter={["!", ["get", "reachable"]]}
    paint={{
      "line-color": "red",
      "line-width": 3,
    }}
    layout={{
      visibility: $populationStyle == "deprived" ? "visible" : "none",
    }}
  />
</GeoJSON>
