<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { emptyGeojson } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend } from "../stores";
  import type { SetRouteInput } from "../types";
  import { type CurrentPOI } from "./stores";

  export let layerName: string;
  export let current: Omit<CurrentPOI, "pt"> | null;
  export let show: boolean;

  let debug = emptyGeojson();
  let fixUnreachable: SetRouteInput | null = null;
  $: updateDebug(current);

  async function updateDebug(current: Omit<CurrentPOI, "pt"> | null) {
    if ($backend && current) {
      if (current.reachable) {
        debug = await $backend.debugReachablePath(current.kind, current.idx);
        fixUnreachable = null;
      } else {
        debug = emptyGeojson();
        fixUnreachable = await $backend.fixUnreachablePOI(
          current.kind,
          current.idx,
        );
      }
    } else {
      debug = emptyGeojson();
      fixUnreachable = null;
    }
  }
</script>

<GeoJSON data={debug}>
  <LineLayer
    {...layerId("debug-reachability-" + layerName)}
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

<GeoJSON data={fixUnreachable?.feature || emptyGeojson()}>
  <LineLayer
    {...layerId("fix-reachability-" + layerName)}
    interactive={false}
    layout={{
      visibility: show ? "visible" : "none",
    }}
    paint={{
      "line-width": 5,
      "line-color": "black",
    }}
  />
</GeoJSON>
