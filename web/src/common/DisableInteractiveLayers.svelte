<script lang="ts">
  import { FillLayer, GeoJSON } from "svelte-maplibre";
  import { interactiveMapLayersEnabled, mode } from "../stores";
  import { layerId } from "./";

  // There are two cases when we want to disable interactions with layers --
  // when StreetView mode is on, block everything. And when in overview
  // or editing, block all reference layers. Achieve this by toggling a
  // layer with a certain z-order.
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
    {...layerId("block-reference-layers")}
    paint={{
      "fill-color": "black",
      "fill-opacity": 0.0,
    }}
    layout={{
      visibility:
        $mode.kind == "edit-route" || $mode.kind == "overview"
          ? "visible"
          : "none",
    }}
  />

  <FillLayer
    {...layerId("block-everything")}
    paint={{
      "fill-color": "yellow",
      "fill-opacity": 0.1,
    }}
    layout={{
      visibility: $interactiveMapLayersEnabled ? "none" : "visible",
    }}
  />
</GeoJSON>
