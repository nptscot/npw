<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    FillLayer,
  } from "svelte-maplibre";
  import LayerControls from "./LayerControls.svelte";
  import { assetUrl } from "../stores";
  import { layerId } from "../common";

  let show = false;
</script>

<LayerControls name="Urban areas" bind:show />

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
