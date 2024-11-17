<script lang="ts">
  import { GeoJSON, hoverStateFilter, FillLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { backend, percent, type IMDZones } from "../stores";
  import { QualitativeLegend } from "../common";
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

  function sum(list: number[]): number {
    return list.reduce((total, x) => total + x, 0);
  }
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
    <QualitativeLegend
      colors={{ Reachable: "purple", "Not reachable": "red" }}
    />
  {/if}
</LayerControls>

<GeoJSON {data} generateId>
  <FillLayer
    manageHoverState
    paint={{
      "fill-color": ["case", ["get", "reachable"], "purple", "red"],
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
</GeoJSON>
