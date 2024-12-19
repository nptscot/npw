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
  import { deprivedPopulation as show } from "./stores";

  let data: DataZones = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      let gj = await $backend.getDataZones();
      // Filter for the top quintile only
      gj.features = gj.features.filter(
        (f) => f.properties.imd_percentile <= 20,
      );
      data = gj;
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

  // Color ramp from https://www.ons.gov.uk/census/maps/choropleth. Lowest value is the worst (darkest).
  let colorScale = ["#080C54", "#186290", "#1F9EB7", "#80C6A3", "#CDE594"];

  // The percentiles are [1, 20]. The 5 colors cover 4 each.
  let limits = [0, 4, 8, 12, 16, 20];
</script>

<LayerControls name="simd">
  <label>
    <input type="checkbox" bind:checked={$show} />
    SIMD
  </label>

  {#if $show}
    <button class="outline" on:click={recalc}>Recalculate</button>
    <p>Only the top 20%ile most deprived zones are shown</p>
    <p>
      {numReachable.toLocaleString()} / {data.features.length.toLocaleString()} zones
      reachable. That's {reachablePopulation.toLocaleString()} / {totalPopulation.toLocaleString()}
      ({percent(reachablePopulation, totalPopulation)}) of the population.
    </p>
    <SequentialLegend {colorScale} {limits} />
    <p>
      Darker colours are more deprived. Zones with a red outline are not
      reachable by the current network.
    </p>
  {/if}
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("simd")}
    manageHoverState
    paint={{
      "fill-color": makeRamp(["get", "imd_percentile"], limits, colorScale),
      "fill-opacity": hoverStateFilter(0.7, 0.9),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
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
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
