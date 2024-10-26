<script lang="ts">
  import {
    VectorTileSource,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { routeNetwork } from "./stores";
  import { assetUrl } from "../stores";
</script>

<VectorTileSource url={`pmtiles://${assetUrl("route_network.pmtiles")}`}>
  <LineLayer
    sourceLayer="rnet"
    manageHoverState
    paint={{
      "line-color": "black",
      "line-width": hoverStateFilter(2, 3),
    }}
    layout={{
      visibility: $routeNetwork ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.all_fastest_bicycle}
    </Popup>
  </LineLayer>
</VectorTileSource>
