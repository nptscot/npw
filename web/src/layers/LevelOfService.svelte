<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { GeoJSON, LineLayer, VectorTileSource } from "svelte-maplibre";
  import { constructMatchExpression, Popup } from "svelte-utils/map";
  import { colorByLoS, levelOfServiceColors } from "../colors";
  import { layerId, QualitativeLegend, roadLineWidth } from "../common";
  import {
    assetUrl,
    backend,
    infraTypeMapping,
    mutationCounter,
  } from "../stores";
  import LayerControls from "./LayerControls.svelte";

  let data: FeatureCollection = {
    type: "FeatureCollection",
    features: [],
  };
  let show = false;
  let showOrig = false;
  let showCurrent = true;

  async function recalc() {
    if ($backend) {
      data = await $backend.renderLevelOfService();
    }
  }

  $: if (show && showCurrent && $mutationCounter > 0) {
    recalc();
  }
</script>

<LayerControls name="Level of Service" bind:show>
  <label>
    <input type="checkbox" bind:checked={showOrig} />
    Show original data
  </label>

  <label>
    <input type="checkbox" bind:checked={showCurrent} />
    Show current derived data
  </label>

  <QualitativeLegend colors={levelOfServiceColors} />
</LayerControls>

<!-- TODO Continue showing this for debugging the map matching -->
<VectorTileSource url={`pmtiles://${assetUrl("cbd.pmtiles")}`}>
  <LineLayer
    {...layerId("los-debug")}
    sourceLayer="cbd_layer"
    paint={{
      "line-color": constructMatchExpression(
        ["get", "Level of Service"],
        {
          High: "mediumseagreen",
          Medium: "orange",
          Low: "red",
          "Should not be used": "brown",
        },
        "cyan",
      ),
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility: show && showOrig ? "visible" : "none",
    }}
  />
</VectorTileSource>

<GeoJSON {data} generateId>
  <LineLayer
    {...layerId("los")}
    layout={{
      visibility: show && showCurrent ? "visible" : "none",
    }}
    paint={{
      "line-width": roadLineWidth(0),
      "line-color": colorByLoS,
      "line-opacity": 0.8,
    }}
    manageHoverState
  >
    <Popup openOn="hover" let:props>
      <table>
        <tr>
          <th>Level of Service</th>
          <td>{props.los}</td>
        </tr>
        <tr>
          <th>Infrastructure type</th>
          <td>{infraTypeMapping[props.infra_type][0]}</td>
        </tr>
        <tr>
          <th>Speed (mph)</th>
          <td>{props.speed}</td>
        </tr>
        <tr>
          <th>Estimated daily traffic volume</th>
          <td>{props.traffic.toLocaleString()}</td>
        </tr>
      </table>
    </Popup>
  </LineLayer>
</GeoJSON>
