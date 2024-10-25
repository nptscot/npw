<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    FillLayer,
  } from "svelte-maplibre";
  import { urbanAreas } from "./stores";
  import { assetUrl } from "../stores";
</script>

<VectorTileSource url={`pmtiles://${assetUrl("urban_areas.pmtiles")}`}>
  <FillLayer
    sourceLayer="urban_areas"
    manageHoverState
    paint={{
      "fill-color": "black",
      "fill-opacity": hoverStateFilter(1.0, 0.5),
    }}
    layout={{
      visibility: $urbanAreas ? "visible" : "none",
    }}
  />
</VectorTileSource>
