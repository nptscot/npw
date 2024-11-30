<script lang="ts">
  import { GeoJSON, CircleLayer } from "svelte-maplibre";
  import { mode, backend } from "../stores";
  import { majorJunctions as show } from "./stores";
  import type { FeatureCollection } from "geojson";

  let data: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.getMajorJunctions();
    }
  }

  $: if ($show && data.features.length == 0) {
    recalc();
  }
</script>

<GeoJSON {data}>
  <CircleLayer
    paint={{
      "circle-color": "purple",
      "circle-radius": 5,
    }}
    layout={{
      visibility: $show && $mode.kind == "edit-route" ? "visible" : "none",
    }}
  />
</GeoJSON>
