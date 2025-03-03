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
  import { deprivedPopulation as show } from "./stores";

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

  $: if ($show && $mutationCounter > 0) {
    recalc();
  }

  // Color ramp from https://www.ons.gov.uk/census/maps/choropleth. Lowest value is the worst (darkest).
  let colorScale = ["#080C54", "#186290", "#1F9EB7", "#80C6A3", "#CDE594"];

  // The percentiles are [1, 20]. The 5 colors cover 4 each.
  let limits = [0, 4, 8, 12, 16, 20];
</script>

<LayerControls name="SIMD" bind:show={$show}>
  <SequentialLegend {colorScale} {limits} />
  <p>
    Darker colours are more deprived. Zones with a red outline are not reachable
    by the current network. Only the top 20%ile most deprived zones are shown.
  </p>
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
