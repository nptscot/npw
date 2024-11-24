<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { backend } from "../stores";
  import type { Feature, Point, FeatureCollection } from "geojson";

  export let hovered: Feature<Point, { reachable: boolean }> | null;

  let debug: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  $: updateDebug(hovered);

  async function updateDebug(
    hovered: Feature<Point, { reachable: boolean }> | null,
  ) {
    if ($backend && hovered) {
      if (hovered.properties.reachable) {
        debug = await $backend.debugReachablePath(hovered.geometry.coordinates);
      } else {
        debug = await $backend.debugUnreachablePath(
          hovered.geometry.coordinates,
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
