<script lang="ts">
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { SequentialLegend } from "svelte-utils";
  import { makeRamp, Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { DataZones } from "../types";
  import LayerControls from "./LayerControls.svelte";
  import { allPopulation as show } from "./stores";

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

  // Color ramp from https://www.ons.gov.uk/census/maps/choropleth. TODO redo for density
  let colorScale = ["#080C54", "#186290", "#1F9EB7", "#80C6A3", "#CDE594"];

  // For density_quintile
  let limits = [0, 1, 2, 3, 4, 5];

  let fillOpacity = hoverStateFilter(
    // @ts-expect-error This really does work
    ["case", ["<", ["get", "density_quintile"], 4], 0.5, 0.0],
    0.9,
  );
</script>

<LayerControls name="Population" bind:show={$show}>
  <SequentialLegend {colorScale} {limits} />
  <p>
    Darker colours are denser. Zones with a red outline are not reachable by the
    current network. Only the top 3 densest quintiles are shown.
  </p>
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("all-population")}
    manageHoverState
    paint={{
      "fill-color": makeRamp(["get", "density_quintile"], limits, colorScale),
      "fill-opacity": fillOpacity,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      <p style:max-width="30vw">
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
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
