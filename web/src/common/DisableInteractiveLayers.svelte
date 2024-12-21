<script lang="ts">
  import { interactiveMapLayersEnabled } from "../stores";
  import { FillLayer, GeoJSON } from "svelte-maplibre";
  import { layerId } from "./";

  // When StreetView is on, disable interactive layers -- no hovering or
  // clicking behavior. Achieve this by enabling a layer on top of
  // everything.
  let coverEverything = {
    type: "Feature" as const,
    properties: {},
    geometry: {
      type: "Polygon" as const,
      coordinates: [
        [
          [180.0, 90.0],
          [-180.0, 90.0],
          [-180.0, -90.0],
          [180.0, -90.0],
          [180.0, 90.0],
        ],
      ],
    },
  };
</script>

<GeoJSON data={coverEverything}>
  <FillLayer
    {...layerId("block-interactiveness")}
    paint={{
      "fill-color": "yellow",
      "fill-opacity": 0.1,
    }}
    layout={{
      visibility: $interactiveMapLayersEnabled ? "none" : "visible",
    }}
  />
</GeoJSON>
