<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import { LineLayer } from "svelte-maplibre";
  import {
    layerId,
    lineColorForDemand,
    lineWidthForDemand,
    roadLineWidth,
  } from "../../common";
  import {
    cyclingDemandHigh,
    cyclingDemandMedium,
    showUncoveredDemand,
    styleCyclingDemand,
  } from "../stores";

  $: filter = $cyclingDemandHigh
    ? ([
        "==",
        ["get", "precalculated_demand_group"],
        "high",
      ] as ExpressionSpecification)
    : $cyclingDemandMedium
      ? ([
          "!=",
          ["get", "precalculated_demand_group"],
          "",
        ] as ExpressionSpecification)
      : undefined;

  // Filter to only show uncovered roads?
  $: opacity = $showUncoveredDemand
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
    visibility: $cyclingDemandHigh || $cyclingDemandMedium ? "visible" : "none",
  }}
  paint={{
    "line-color": $styleCyclingDemand
      ? lineColorForDemand(["get", "precalculated_demand"])
      : "grey",
    "line-opacity": opacity,
    "line-width": $styleCyclingDemand
      ? lineWidthForDemand(["get", "precalculated_demand"])
      : roadLineWidth(4),
  }}
/>
