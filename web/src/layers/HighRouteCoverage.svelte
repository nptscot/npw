<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { backend, type PrecalculatedFlows } from "../stores";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { highRouteCoverage as show } from "./stores";
  import { percent, lineWidthForDemand, lineColorForDemand } from "../utils";
  import type { ExpressionSpecification } from "maplibre-gl";

  let onlyCovered = false;
  let onlyQuintile: string = "";

  let data: PrecalculatedFlows = {
    type: "FeatureCollection",
    features: [],
    quintile_sums: [],
    covered_quintile_sums: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.renderPrecalculatedFlows();
    }
  }

  $: if ($show && data.features.length == 0) {
    recalc();
  }

  function makeFilter(
    onlyCovered: boolean,
    onlyQuintile: string,
  ): ExpressionSpecification {
    let filter: ExpressionSpecification = ["all"];
    if (onlyCovered) {
      filter.push(["get", "covered"]);
    }
    if (onlyQuintile.length > 0) {
      filter.push(["==", ["get", "quintile"], parseInt(onlyQuintile)]);
    }
    return filter;
  }
</script>

<LayerControls name="high npt route coverage">
  <label>
    <input type="checkbox" bind:checked={$show} />
    High NPT route coverage
  </label>

  {#if $show}
    <div style="border: 1px solid black; padding: 4px">
      <button class="outline" on:click={recalc}>Recalculate</button>

      <label>
        <input type="checkbox" bind:checked={onlyCovered} />
        Only show routes covered by current edits
      </label>

      <label>
        Only show quintile:
        <select bind:value={onlyQuintile}>
          <option value="">Show all</option>
          <option value="1">1</option>
          <option value="2">2</option>
          <option value="3">3</option>
          <option value="4">4</option>
          <option value="5">5</option>
        </select>
      </label>

      {#if data.quintile_sums.length > 0}
        <p>Flow covered by current edits</p>
        <ul>
          <li>
            Quintile 1: {percent(
              data.covered_quintile_sums[0],
              data.quintile_sums[0],
            )}
          </li>
          <li>
            Quintile 2: {percent(
              data.covered_quintile_sums[1],
              data.quintile_sums[1],
            )}
          </li>
          <li>
            Quintile 3: {percent(
              data.covered_quintile_sums[2],
              data.quintile_sums[2],
            )}
          </li>
          <li>
            Quintile 4: {percent(
              data.covered_quintile_sums[3],
              data.quintile_sums[3],
            )}
          </li>
          <li>
            Quintile 5: {percent(
              data.covered_quintile_sums[4],
              data.quintile_sums[4],
            )}
          </li>
        </ul>
      {/if}
    </div>
  {/if}
</LayerControls>

<GeoJSON {data}>
  <LineLayer
    layout={{
      visibility: $show ? "visible" : "none",
    }}
    filter={makeFilter(onlyCovered, onlyQuintile)}
    paint={{
      "line-width": lineWidthForDemand("flow"),
      "line-color": lineColorForDemand("flow"),
    }}
  >
    <Popup openOn="hover" let:props>
      Flow {props.flow.toLocaleString()} is in quintile {props.quintile},
      covered {props.covered}
    </Popup>
  </LineLayer>
</GeoJSON>
