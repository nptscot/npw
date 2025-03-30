<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { emptyGeojson } from "svelte-utils/map";
  import { tierColors } from "../colors";
  import { layerId } from "../common";
  import { backend } from "../stores";
  import type { PoiKind, SetRouteInput } from "../types";

  // For town centres and settlements and POIs
  interface SimplePOI {
    kind: PoiKind;
    idx: number;
    reachable: boolean;
  }

  export let layerName: string;
  export let current: SimplePOI | null;
  export let show: boolean;

  let debug = emptyGeojson();
  let fixUnreachable: SetRouteInput | null = null;
  $: updateDebug(current);

  async function updateDebug(current: SimplePOI | null) {
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
      "line-color": tierColors.LocalAccess,
      "line-dasharray": [2, 1],
    }}
  />
</GeoJSON>
