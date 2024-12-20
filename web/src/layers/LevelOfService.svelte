<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import {
    VectorTileSource,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { backend, infraTypeMapping, assetUrl } from "../stores";
  import { Popup, constructMatchExpression } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { layerId, QualitativeLegend } from "../common";
  import { levelOfServiceColors, colorByLoS } from "../colors";

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

  $: if (show && showCurrent && data.features.length == 0) {
    recalc();
  }
</script>

<LayerControls name="level of service">
  <label>
    <input type="checkbox" bind:checked={show} />
    Level of Service
  </label>

  {#if show}
    <button class="outline" on:click={recalc}>Recalculate</button>
    <label>
      <input type="checkbox" bind:checked={showOrig} />
      Show original data
    </label>

    <label>
      <input type="checkbox" bind:checked={showCurrent} />
      Show current derived data
    </label>

    <QualitativeLegend colors={levelOfServiceColors} />
  {/if}
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
      "line-width": 10,
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
      "line-width": hoverStateFilter(5, 7),
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
