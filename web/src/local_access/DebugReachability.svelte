<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { emptyGeojson } from "svelte-utils/map";
  import { tierColors } from "../colors";
  import { layerId } from "../common";
  import { backend } from "../stores";
  import {
    debugReachabilityCurrentPOI,
    fixCurrentPOI,
    type POI,
  } from "./stores";

  export let current: POI | null;

  $: updateDebug(current);
  async function updateDebug(current: POI | null) {
    $debugReachabilityCurrentPOI = null;
    $fixCurrentPOI = null;

    if ($backend && current) {
      if (current.reachable) {
        // If this returns an error, then the POI is directly on the network
        try {
          $debugReachabilityCurrentPOI = await $backend.debugReachablePath(
            current.kind,
            current.idx,
          );
        } catch (err) {}
      } else {
        $fixCurrentPOI = await $backend.fixUnreachablePOI(
          current.kind,
          current.idx,
        );
      }
    }
  }
</script>

<GeoJSON data={$debugReachabilityCurrentPOI || emptyGeojson()}>
  <LineLayer
    {...layerId("debug-reachability-pois")}
    interactive={false}
    paint={{
      "line-width": 3,
      "line-color": "blue",
      "line-dasharray": [2, 1],
    }}
  />
</GeoJSON>

<GeoJSON data={$fixCurrentPOI || emptyGeojson()}>
  <LineLayer
    {...layerId("fix-reachability-pois")}
    interactive={false}
    paint={{
      "line-width": 5,
      "line-color": tierColors.LocalAccess,
      "line-dasharray": [2, 1],
    }}
  />
</GeoJSON>
