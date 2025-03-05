<script lang="ts">
  import type { Feature, MultiPolygon, Point } from "geojson";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { emptyGeojson } from "svelte-utils/map";
  import { layerId } from "../common";
  import { backend } from "../stores";
  import type { PoiKind, SetRouteInput } from "../types";
  import { severances } from "./stores";

  export let layerName: string;
  export let hovered: Feature<
    Point | MultiPolygon,
    { poi_kind: PoiKind; reachable: boolean; idx: number }
  > | null;

  let debug = emptyGeojson();
  let fixUnreachable: SetRouteInput | null = null;
  $: updateDebug(hovered);

  async function updateDebug(
    hovered: Feature<
      Point | MultiPolygon,
      { poi_kind: PoiKind; reachable: boolean; idx: number }
    > | null,
  ) {
    $severances = false;
    if ($backend && hovered) {
      if (hovered.properties.reachable) {
        debug = await $backend.debugReachablePath(
          hovered.properties.poi_kind,
          hovered.properties.idx,
        );
        fixUnreachable = null;
      } else {
        debug = emptyGeojson();
        fixUnreachable = await $backend.fixUnreachablePOI(
          hovered.properties.poi_kind,
          hovered.properties.idx,
        );
        $severances = true;
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
    paint={{
      "line-width": 5,
      "line-color": "black",
    }}
  />
</GeoJSON>
