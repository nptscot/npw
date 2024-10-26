<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    FillLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { assetUrl } from "../stores";

  let show = false;
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Town centres
  </label>
</LayerControls>

<VectorTileSource url={`pmtiles://${assetUrl("town_centres.pmtiles")}`}>
  <FillLayer
    sourceLayer="town_centres"
    manageHoverState
    paint={{
      "fill-color": "black",
      "fill-opacity": hoverStateFilter(1.0, 0.5),
    }}
    layout={{
      visibility: show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.name}
    </Popup>
  </FillLayer>
</VectorTileSource>
