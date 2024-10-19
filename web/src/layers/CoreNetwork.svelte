<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { coreNetwork } from "./stores";
  import { assetUrl } from "../stores";
</script>

<GeoJSON data={assetUrl("core_network.geojson")} generateId>
  <LineLayer
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
      {props.name_1} has function {props.road_function} and Go Dutch value {props.all_fastest_bicycle_go_dutch}
    </Popup>
  </LineLayer>
</GeoJSON>
