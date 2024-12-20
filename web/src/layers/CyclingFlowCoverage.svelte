<script lang="ts">
  import { layerId } from "../common";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { backend, type PrecalculatedFlows } from "../stores";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { percent, lineWidthForDemand } from "../utils";
  import type { ExpressionSpecification } from "maplibre-gl";

  export let show: boolean;
  export let quintile: number;
  export let label: string;

  let onlyCovered = false;

  let data: PrecalculatedFlows = {
    type: "FeatureCollection",
    features: [],
    covered_quintile_sums: [],
    total_quintile_sums: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.renderPrecalculatedFlows();
    }
  }

  $: if (show && data.features.length == 0) {
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
  <button class="outline" on:click={recalc}>Recalculate</button>

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
