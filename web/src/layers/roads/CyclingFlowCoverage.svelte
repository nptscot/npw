<script lang="ts">
  import { LineLayer } from "svelte-maplibre";
  import { layerId, roadLineWidth } from "../../common";
  import { cyclingFlow1, cyclingFlow2, cyclingFlow3 } from "../stores";

  // Depending on the current tier, show one quintile of precalculated
  // cycling flow. Filter to only show uncovered roads.

  $: quintile = $cyclingFlow1
    ? 1
    : $cyclingFlow2
      ? 2
      : $cyclingFlow3
        ? 3
        : null;
</script>

<LineLayer
  {...layerId("uncovered-cycling-flows")}
  filter={quintile == null
    ? ["boolean", false]
    : ["==", ["get", "precalculated_flow_quintile"], quintile]}
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
