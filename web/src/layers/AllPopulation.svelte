<script lang="ts">
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { makeRamp, Popup } from "svelte-utils/map";
  import { population } from "../colors";
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
      data = await $backend.getDataZones();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($populationStyle == "population" && $mutationCounter > 0) {
    recalc();
  }

  let fillOpacity = hoverStateFilter(
    // @ts-expect-error This really does work
    ["case", ["<", ["get", "density_quintile"], 4], 0.5, 0.0],
    0.9,
  );
</script>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("all-population")}
    manageHoverState
    paint={{
      "fill-color": makeRamp(
        ["get", "density_quintile"],
        population.limits,
        population.colorScale,
      ),
      "fill-opacity": fillOpacity,
    }}
    layout={{
      visibility: $populationStyle == "population" ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      <p style:max-width="200px">
        Data zone {props.id}
        {props.reachable ? "is" : "is not"} reachable. It has {props.population.toLocaleString()}
        people, with a density of {Math.round(
          props.population / props.area_km2,
        ).toLocaleString()} people per square kilometer, putting it in quintile {props.density_quintile}
        for this study area.
      </p>
    </Popup>
  </FillLayer>

  <LineLayer
    {...layerId("all-population-outline")}
    filter={["!", ["get", "reachable"]]}
    paint={{
      "line-color": "red",
      "line-width": 3,
    }}
    layout={{
      visibility: $populationStyle == "population" ? "visible" : "none",
    }}
  />
</GeoJSON>
