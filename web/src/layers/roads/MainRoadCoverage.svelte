<script lang="ts">
  import { LineLayer } from "svelte-maplibre";
  import { layerId, roadLineWidth } from "../../common";
  import { mainRoadCoverage } from "../stores";
</script>

<LineLayer
  {...layerId("uncovered-main-roads")}
  filter={["get", "is_main_road"]}
  layout={{
    visibility: $mainRoadCoverage ? "visible" : "none",
  }}
  paint={{
    "line-color": "grey",
    "line-opacity": [
      "case",
      ["to-boolean", ["feature-state", "current_infra"]],
      0.0,
      1.0,
    ],
    "line-width": roadLineWidth(4),
  }}
/>
