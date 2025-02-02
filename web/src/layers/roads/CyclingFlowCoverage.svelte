<script lang="ts">
  import { LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, roadLineWidth } from "../../common";
  import {
    cyclingFlow1,
    cyclingFlow2,
    cyclingFlow3,
    debugAllCyclingFlow,
  } from "../stores";

  // Depending on the current tier, show one quintile of precalculated
  // cycling flow. Filter to only show uncovered roads.

  $: quintile = $debugAllCyclingFlow
    ? null
    : $cyclingFlow1
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
    ? undefined
    : ["==", ["get", "precalculated_flow_quintile"], quintile]}
  layout={{
    visibility:
      $cyclingFlow1 || $cyclingFlow2 || $cyclingFlow3 || $debugAllCyclingFlow
        ? "visible"
        : "none",
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
>
  {#if $debugAllCyclingFlow}
    <Popup openOn="hover" let:props>
      <p>
        Flow {props.precalculated_flow.toLocaleString()}, quintile {props.precalculated_flow_quintile}
      </p>
    </Popup>
  {/if}
</LineLayer>
