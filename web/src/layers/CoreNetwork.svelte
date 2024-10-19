<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { coreNetwork } from "./stores";

  // TODO Rethink hosting
  let url =
    "https://github.com/nptscot/npt/releases/download/CN_class/city_of_edinburgh_2024-10-01_4_coherent_network.geojson";
</script>

<GeoJSON data={url} generateId>
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
