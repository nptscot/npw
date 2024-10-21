<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { coreNetwork } from "./stores";
  import { assetUrl } from "../stores";
</script>

<VectorTileSource url={`pmtiles://${assetUrl("core_network.pmtiles")}`}>
  <LineLayer
    sourceLayer="coherent_networks"
    manageHoverState
    paint={{
      "line-color": "black",
      "line-width": hoverStateFilter(2, 3),
    }}
    layout={{
      visibility: $coreNetwork ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      Go Dutch value {props.all_fastest_bicycle_go_dutch}
    </Popup>
  </LineLayer>
</VectorTileSource>
