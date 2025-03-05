<script lang="ts">
  import { LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, lineWidthForDemand, roadLineWidth } from "../../common";
  import {
    cyclingDemand1,
    cyclingDemand2,
    debugAllCyclingDemand,
    debugCyclingDemandMin,
  } from "../stores";

  // Depending on the current tier, show one quintile of precalculated cycling
  // demand. Filter to only show uncovered roads.

  $: quintile = $debugAllCyclingDemand
    ? null
    : $cyclingDemand1
      ? 1
      : $cyclingDemand2
        ? 2
        : null;
</script>

<LineLayer
  {...layerId("uncovered-cycling-demands")}
  filter={quintile == null
    ? [">=", ["get", "precalculated_demand"], $debugCyclingDemandMin]
    : ["==", ["get", "precalculated_demand_quintile"], quintile]}
  layout={{
    visibility: $cyclingDemand1 || $cyclingDemand2 ? "visible" : "none",
  }}
  paint={{
    "line-color": "grey",
    "line-opacity": [
      "case",
      ["to-boolean", ["feature-state", "current_infra"]],
      0.0,
      1.0,
    ],
    "line-width": $debugAllCyclingDemand
      ? lineWidthForDemand(["get", "precalculated_demand"])
      : roadLineWidth(4),
  }}
>
  {#if $debugAllCyclingDemand}
    <Popup openOn="hover" let:props>
      <p>
        Demand {props.precalculated_demand.toLocaleString()}, quintile {props.precalculated_demand_quintile}
      </p>
    </Popup>
  {/if}
</LineLayer>
