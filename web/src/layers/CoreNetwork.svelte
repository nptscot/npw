<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { assetUrl } from "../stores";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Core network
  </label>
</LayerControls>

<VectorTileSource url={`pmtiles://${assetUrl("core_network.pmtiles")}`}>
  <LineLayer
    sourceLayer="coherent_networks"
    manageHoverState
    paint={{
      "line-color": "black",
      "line-width": hoverStateFilter(2, 3),
    }}
    layout={{
      visibility: show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      Go Dutch value {props.all_fastest_bicycle_go_dutch}
    </Popup>
  </LineLayer>
</VectorTileSource>
