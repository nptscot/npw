<script lang="ts">
  import { LineLayer, VectorTileSource } from "svelte-maplibre";
  import { constructMatchExpression, makeRamp, Popup } from "svelte-utils/map";
  import { speed } from "../../colors";
  import { layerId, roadLineWidth } from "../../common";
  import { assetUrl, roadStyle } from "../../stores";
  import { debugOriginalData } from "./stores";
</script>

<VectorTileSource url={`pmtiles://${assetUrl("cbd.pmtiles")}`}>
  <LineLayer
    {...layerId("traffic-debug")}
    sourceLayer="cbd_layer"
    filter={["has", "Traffic volume category"]}
    paint={{
      "line-color": constructMatchExpression(
        ["get", "Traffic volume category"],
        {
          "0 to 1999": "#27918d",
          "2000 to 3999": "#ffaa33",
          "4000+": "#440154",
        },
        "cyan",
      ),
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility:
        $roadStyle == "traffic" && $debugOriginalData ? "visible" : "none",
    }}
  />

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
      visibility:
        $roadStyle == "los" && $debugOriginalData ? "visible" : "none",
    }}
  />

  <LineLayer
    {...layerId("existing-infra-debug")}
    sourceLayer="cbd_layer"
    filter={["!=", ["get", "Infrastructure type"], "Mixed Traffic Street"]}
    paint={{
      "line-color": constructMatchExpression(
        ["get", "Infrastructure type"],
        {
          "Segregated Track (wide)": "#054d05",
          "Off Road Cycleway": "#3a9120",
          "Segregated Track (narrow)": "#87d668",
          "Shared Footway": "#ffbf00",
          "Painted Cycle Lane": "#FF0000",
        },
        "cyan",
      ),
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility:
        $roadStyle == "existing_infra" && $debugOriginalData
          ? "visible"
          : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      osmactive: {props["Infrastructure type"]}
    </Popup>
  </LineLayer>

  <LineLayer
    {...layerId("speed_limit-debug")}
    sourceLayer="cbd_layer"
    paint={{
      "line-color": makeRamp(
        ["get", "Speed limit"],
        speed.limits,
        speed.colorScale,
      ),
      "line-width": roadLineWidth(4),
    }}
    layout={{
      visibility:
        $roadStyle == "speed" && $debugOriginalData ? "visible" : "none",
    }}
  />
</VectorTileSource>
