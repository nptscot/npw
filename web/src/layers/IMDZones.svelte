<script lang="ts">
  import {
    GeoJSON,
    hoverStateFilter,
    FillLayer,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup, makeColorRamp } from "svelte-utils/map";
  import { SequentialLegend } from "svelte-utils";
  import LayerControls from "./LayerControls.svelte";
  import { backend, type IMDZones } from "../stores";
  import { percent, sum } from "../utils";
  import { imdZones as show } from "./stores";

  let data: IMDZones = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.getIMDZones();
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

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={$show} />
    SIMD
  </label>

  {#if $show}
    <button on:click={recalc}>Recalculate</button>
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
    manageHoverState
    paint={{
      "fill-color": makeColorRamp(["get", "percentile"], limits, colorScale),
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
        people, and a SIMD rank of {props.rank}, putting it in the {props.percentile}
        percentile.
      </p>
    </Popup>
  </FillLayer>

  <LineLayer
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
