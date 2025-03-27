<script lang="ts">
  import { FillLayer, GeoJSON, LineLayer } from "svelte-maplibre";
  import { makeRamp } from "svelte-utils/map";
  import { meshDensity } from "../colors";
  import { layerId } from "../common";
  import { backend, mutationCounter } from "../stores";
  import type { GridMeshDensity } from "../types";
  import { gridMeshDensity as show } from "./stores";

  let data: GridMeshDensity = {
    type: "FeatureCollection",
    features: [],
  };

  let resolution = 800;
  let xOffset = 0;
  let yOffset = 0;

  // Don't use lastUpdate pattern here, since there are other cache keys here
  async function recalc(resolution: number, xOffset: number, yOffset: number) {
    if ($backend) {
      data = await $backend.getGridMeshDensity(
        resolution,
        xOffset * resolution,
        yOffset * resolution,
      );
    }
  }

  $: if ($show && $mutationCounter > 0) {
    recalc(resolution, xOffset, yOffset);
  }
</script>

<GeoJSON {data} generateId>
  <FillLayer
    {...layerId("mesh-density-grid")}
    paint={{
      "fill-color": makeRamp(
        ["get", "routes"],
        meshDensity.limits,
        meshDensity.colorScale,
      ),
      "fill-opacity": 0.5,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />

  <LineLayer
    {...layerId("mesh-density-grid-outline")}
    paint={{
      "line-color": "black",
      "line-width": 1,
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  />
</GeoJSON>
