<script lang="ts">
  import {
    FillLayer,
    hoverStateFilter,
    VectorTileSource,
  } from "svelte-maplibre";
  import { layerId } from "../common";
  import { assetUrl } from "../stores";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
</script>

<LayerControls name="Urban areas" bind:show empty />

<VectorTileSource url={`pmtiles://${assetUrl("urban_areas.pmtiles")}`}>
  <FillLayer
    {...layerId("urban-areas")}
    sourceLayer="urban_areas"
    manageHoverState
    paint={{
      "fill-color": "black",
      "fill-opacity": hoverStateFilter(1.0, 0.5),
    }}
    layout={{
      visibility: show ? "visible" : "none",
    }}
  />
</VectorTileSource>
