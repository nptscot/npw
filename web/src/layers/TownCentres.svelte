<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    FillLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { townCentres } from "./stores";
  import { assetUrl } from "../stores";
</script>

<VectorTileSource url={`pmtiles://${assetUrl("town_centres.pmtiles")}`}>
  <FillLayer
    sourceLayer="town_centres"
    manageHoverState
    paint={{
      "fill-color": "black",
      "fill-opacity": hoverStateFilter(1.0, 0.5),
    }}
    layout={{
      visibility: $townCentres ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.name}
    </Popup>
  </FillLayer>
</VectorTileSource>
