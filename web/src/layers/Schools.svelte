<script lang="ts">
  import { GeoJSON, hoverStateFilter, CircleLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { schools } from "./stores";
  import { assetUrl } from "../stores";
</script>

<GeoJSON data={assetUrl("schools.geojson")} generateId>
  <CircleLayer
    manageHoverState
    paint={{
      "circle-color": "black",
      "circle-radius": hoverStateFilter(5, 8),
    }}
    layout={{
      visibility: $schools ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.name} is a {props.type} school with {props.pupils} pupils
    </Popup>
  </CircleLayer>
</GeoJSON>
