<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import { LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import {
    layerId,
    lineColorForDemand,
    lineWidthForDemand,
  } from "../../common";
  import {
    cyclingDemand1,
    cyclingDemand2,
    debugAllCyclingDemand,
    debugCyclingDemandMin,
    showUncovered,
  } from "../stores";

  $: filter = $debugAllCyclingDemand
    ? ([
        ">=",
        ["get", "precalculated_demand"],
        $debugCyclingDemandMin,
      ] as ExpressionSpecification)
    : $cyclingDemand1
      ? ([
          "==",
          ["get", "precalculated_demand_group"],
          "high",
        ] as ExpressionSpecification)
      : $cyclingDemand2
        ? ([
            "==",
            ["get", "precalculated_demand_group"],
            "medium",
          ] as ExpressionSpecification)
        : undefined;

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
  {filter}
  layout={{
    visibility: $cyclingDemand1 || $cyclingDemand2 ? "visible" : "none",
  }}
  paint={{
    "line-color": lineColorForDemand(["get", "precalculated_demand"]),
    "line-opacity": opacity,
    "line-width": lineWidthForDemand(["get", "precalculated_demand"]),
  }}
>
  {#if $debugAllCyclingDemand}
    <Popup openOn="hover" let:props>
      <p>
        Demand {props.precalculated_demand.toLocaleString()}, group {props.precalculated_demand_group}
      </p>
    </Popup>
  {/if}
</LineLayer>
