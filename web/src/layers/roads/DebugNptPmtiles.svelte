<script lang="ts">
  import { LineLayer, VectorTileSource } from "svelte-maplibre";
  import { constructMatchExpression, makeRamp, Popup } from "svelte-utils/map";
  import { nptStreetSpaceColors, speed } from "../../colors";
  import { layerId, roadLineWidth } from "../../common";
  import { assetUrl, referenceRoadStyle } from "../../stores";
  import { debugOriginalData } from "../stores";
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
      "line-width": roadLineWidth(0),
    }}
    layout={{
      visibility:
        $referenceRoadStyle == "traffic" && $debugOriginalData
          ? "visible"
          : "none",
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
      "line-width": roadLineWidth(1),
    }}
    layout={{
      visibility:
        $referenceRoadStyle == "los" && $debugOriginalData ? "visible" : "none",
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
          "Painted Cycle Lane": "#7faedd",
        },
        "cyan",
      ),
      "line-width": roadLineWidth(1),
    }}
    layout={{
      visibility:
        $referenceRoadStyle == "existing_infra" && $debugOriginalData
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
      "line-width": roadLineWidth(1),
    }}
    layout={{
      visibility:
        $referenceRoadStyle == "speed" && $debugOriginalData
          ? "visible"
          : "none",
    }}
  />
</VectorTileSource>

<VectorTileSource url={`pmtiles://${assetUrl("core_network.pmtiles")}`}>
  <LineLayer
    {...layerId("cn-debug")}
    sourceLayer="coherent_networks"
    filter={["!=", ["get", "road_function_npt"], "Local Access"]}
    paint={{
      "line-color": constructMatchExpression(
        ["get", "road_function_npt"],
        {
          Primary: "#c00000",
          Secondary: "#e97132",
        },
        "cyan",
      ),
      "line-width": roadLineWidth(1),
    }}
    layout={{
      visibility:
        $referenceRoadStyle == "cn" && $debugOriginalData ? "visible" : "none",
    }}
  />
</VectorTileSource>

<VectorTileSource url={`pmtiles://${assetUrl("streetspace.pmtiles")}`}>
  <LineLayer
    {...layerId("street_space-debug")}
    sourceLayer="street_space"
    paint={{
      "line-color": constructMatchExpression(
        ["get", "carriageway_2way"],
        nptStreetSpaceColors,
        "cyan",
      ),
      "line-width": roadLineWidth(1),
    }}
    layout={{
      visibility:
        $referenceRoadStyle == "street_space" && $debugOriginalData
          ? "visible"
          : "none",
    }}
  />
</VectorTileSource>
