<script lang="ts">
  import { GeoJSON, hoverStateFilter, CircleLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { gpHospitals } from "./stores";
  import { assetUrl } from "../stores";
</script>

<GeoJSON data={assetUrl("gp_practices.geojson")} generateId>
  <CircleLayer
    manageHoverState
    paint={{
      "circle-color": "black",
      "circle-radius": hoverStateFilter(5, 8),
    }}
    layout={{
      visibility: $gpHospitals ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.name} is a GP practice
    </Popup>
  </CircleLayer>
</GeoJSON>

<GeoJSON data={assetUrl("hospitals.geojson")} generateId>
  <CircleLayer
    manageHoverState
    paint={{
      "circle-color": "red",
      "circle-radius": hoverStateFilter(5, 8),
    }}
    layout={{
      visibility: $gpHospitals ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.name} is a hospital
    </Popup>
  </CircleLayer>
</GeoJSON>
