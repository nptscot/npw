<script lang="ts">
  import type { DataDrivenPropertyValueSpecification } from "maplibre-gl";
  import { LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, lineWidthForDemand, roadLineWidth } from "../../common";
  import {
    cyclingDemand1,
    cyclingDemand2,
    debugAllCyclingDemand,
    debugCyclingDemandMin,
    showUncovered,
  } from "../stores";

  // Depending on the current tier, show one quintile of precalculated cycling
  // demand.

  $: quintile = $debugAllCyclingDemand
    ? null
    : $cyclingDemand1
      ? 1
      : $cyclingDemand2
        ? 2
        : null;

  // Filter to only show uncovered roads?
  $: opacity = $showUncovered
    ? 1.0
    : ([
        "case",
        ["to-boolean", ["feature-state", "current_infra"]],
        0.0,
        1.0,
      ] as DataDrivenPropertyValueSpecification<number>);
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
    "line-opacity": opacity,
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
