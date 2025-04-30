<script lang="ts">
  import type { DataDrivenPropertyValueSpecification } from "maplibre-gl";
  import { LineLayer } from "svelte-maplibre";
  import { layerId, roadLineWidth } from "../../common";
  import { arterialRoadCoverage, showUncoveredDemand } from "../stores";

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
  {...layerId("uncovered-main-roads")}
  filter={["all", ["get", "is_arterial_road"], ["get", "within_settlement"]]}
  layout={{
    visibility: $arterialRoadCoverage ? "visible" : "none",
  }}
  paint={{
    "line-color": "grey",
    "line-opacity": opacity,
    "line-width": roadLineWidth(4),
  }}
/>
