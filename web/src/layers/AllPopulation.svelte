<script lang="ts">
  import { layerId } from "../common";
  import {
    GeoJSON,
    hoverStateFilter,
    FillLayer,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup, makeRamp } from "svelte-utils/map";
  import { SequentialLegend } from "svelte-utils";
  import LayerControls from "./LayerControls.svelte";
  import { backend, type DataZones } from "../stores";
  import { percent, sum } from "../utils";
  import { allPopulation as show } from "./stores";

  let data: DataZones = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.getDataZones();
    }
  }

  $: if ($show && data.features.length == 0) {
    recalc();
  }

  $: totalPopulation = sum(data.features.map((f) => f.properties.population));
  $: numReachable = data.features.filter((f) => f.properties.reachable).length;
  $: reachablePopulation = sum(
    data.features
      .filter((f) => f.properties.reachable)
      .map((f) => f.properties.population),
  );

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

<LayerControls name="population density">
  <label>
    <input type="checkbox" bind:checked={$show} />
    Population
  </label>

  {#if $show}
    <button class="outline" on:click={recalc}>Recalculate</button>
    <p>
      {numReachable.toLocaleString()} / {data.features.length.toLocaleString()} zones
      reachable. That's {reachablePopulation.toLocaleString()} / {totalPopulation.toLocaleString()}
      ({percent(reachablePopulation, totalPopulation)}) of the population.
    </p>
    <SequentialLegend {colorScale} {limits} />
    <p>
      Darker colours are denser. Zones with a red outline are not reachable by
      the current network. Only the top 3 densest quintiles are shown.
    </p>
  {/if}
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
