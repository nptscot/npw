<script lang="ts">
  import { layerId } from "../common";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { backend } from "../stores";
  import type {
    Feature,
    Point,
    MultiPolygon,
    FeatureCollection,
  } from "geojson";

  export let kind: string;
  export let hovered: Feature<
    Point | MultiPolygon,
    { reachable: boolean; idx: number }
  > | null;

  let debug: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  $: updateDebug(hovered);

  async function updateDebug(
    hovered: Feature<
      Point | MultiPolygon,
      { reachable: boolean; idx: number }
    > | null,
  ) {
    if ($backend && hovered) {
      if (hovered.properties.reachable) {
        debug = await $backend.debugReachablePath(kind, hovered.properties.idx);
      } else {
        debug = await $backend.debugUnreachablePath(
          kind,
          hovered.properties.idx,
        );
      }
    } else {
      debug = {
        type: "FeatureCollection",
        features: [],
      };
    }
  }
</script>

<GeoJSON data={debug} generateId>
  <LineLayer
    {...layerId("debug-reachability")}
    paint={{
      "line-width": 3,
      "line-color": [
        "case",
        ["==", ["get", "kind"], "severance"],
        "red",
        "blue",
      ],
    }}
  />
</GeoJSON>
