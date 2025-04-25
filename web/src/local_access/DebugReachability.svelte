<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { emptyGeojson } from "svelte-utils/map";
  import { tierColors } from "../colors";
  import { layerId } from "../common";
  import { backend } from "../stores";
  import { fixCurrentPOI, type POI } from "./stores";

  export let current: POI | null;
  export let show: boolean;

  let debug = emptyGeojson();
  $: updateDebug(current);

  async function updateDebug(current: POI | null) {
    if ($backend && current) {
      if (current.reachable) {
        debug = await $backend.debugReachablePath(current.kind, current.idx);
        $fixCurrentPOI = null;
      } else {
        debug = emptyGeojson();
        $fixCurrentPOI = await $backend.fixUnreachablePOI(
          current.kind,
          current.idx,
        );
      }
    } else {
      debug = emptyGeojson();
      $fixCurrentPOI = null;
    }
  }
</script>

<GeoJSON data={debug}>
  <LineLayer
    {...layerId("debug-reachability-pois")}
    interactive={false}
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "line-width": 3,
      "line-color": "blue",
    }}
  />
</GeoJSON>

<GeoJSON data={$fixCurrentPOI || emptyGeojson()}>
  <LineLayer
    {...layerId("fix-reachability-pois")}
    interactive={false}
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "line-width": 5,
      "line-color": tierColors.LocalAccess,
      "line-dasharray": [2, 1],
    }}
  />
</GeoJSON>
