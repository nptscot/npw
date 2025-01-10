<script lang="ts">
  import type { ExpressionSpecification } from "maplibre-gl";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend, mutationCounter, type PrecalculatedFlows } from "../stores";
  import { lineWidthForDemand, percent } from "../utils";
  import LayerControls from "./LayerControls.svelte";

  export let show: boolean;
  export let quintile: number;
  export let label: string;

  let onlyCovered = false;

  let lastUpdate = 0;
  let data: PrecalculatedFlows = {
    type: "FeatureCollection",
    features: [],
    covered_quintile_sums: [],
    total_quintile_sums: [],
  };

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      data = await $backend.renderPrecalculatedFlows();
      lastUpdate = $mutationCounter;
    }
  }

  $: if (show && $mutationCounter > 0) {
    recalc();
  }

  function makeFilter(onlyCovered: boolean): ExpressionSpecification {
    let filter: ExpressionSpecification = [
      "all",
      ["==", ["get", "quintile"], quintile],
    ];
    if (onlyCovered) {
      filter.push(["get", "covered"]);
    }
    return filter;
  }
</script>

<LayerControls name={label + " cycling flow"} bind:show>
  <label>
    <input type="checkbox" bind:checked={onlyCovered} />
    Only show routes covered by current edits
  </label>

  {#if data.total_quintile_sums.length > 0}
    <p>
      Quintile {quintile} flow covered by current edits:
      {percent(
        data.covered_quintile_sums[quintile - 1],
        data.total_quintile_sums[quintile - 1],
      )}
    </p>
  {/if}
</LayerControls>

<GeoJSON {data}>
  <LineLayer
    {...layerId("cycling-flow-" + quintile)}
    layout={{
      visibility: show ? "visible" : "none",
    }}
    filter={makeFilter(onlyCovered)}
    paint={{
      "line-width": lineWidthForDemand("flow"),
      "line-color": "grey",
    }}
  >
    <Popup openOn="hover" let:props>
      Flow {props.flow.toLocaleString()} is in quintile {props.quintile},
      covered {props.covered}
    </Popup>
  </LineLayer>
</GeoJSON>
