<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { constructMatchExpression, makeRamp, Popup } from "svelte-utils/map";
  import { infraTypeColors, tierColors } from "../../colors";
  import { layerId, roadLineWidth } from "../../common";
  import { backend, roadStyle, type RoadStyle } from "../../stores";
  import { infraTypeMapping } from "../../types";
  import { lineColorForGradient } from "../../utils";

  function makeFilter(style: RoadStyle): ExpressionSpecification | undefined {
    if (style == "cn") {
      return ["to-boolean", ["get", "cn"]];
    } else if (style == "existing_infra") {
      return ["to-boolean", ["get", "existing_infra"]];
    }
    return undefined;
  }

  function lineColor(
    style: RoadStyle,
  ): DataDrivenPropertyValueSpecification<string> {
    if (style == "cn") {
      return constructMatchExpression(["get", "cn"], tierColors, "cyan");
    } else if (style == "existing_infra") {
      return constructMatchExpression(
        ["get", "existing_infra"],
        infraTypeColors,
        "red",
      );
    } else if (style == "traffic") {
      // TODO Copied
      let colorScale = ["#27918d", "#ffaa33", "#440154"];
      let limits = [0, 2000, 4000, 10000];
      return makeRamp(["get", "traffic"], limits, colorScale);
    } else if (style == "gradient") {
      return lineColorForGradient();
    } else if (style == "speed") {
      // TODO Copied
      let colorScale = [
        "#8a9a5b",
        "#ffc300",
        "#cc5500",
        "#c70039",
        "#900c3f",
        "#581845",
      ];
      let limits = [20, 30, 40, 50, 60, 70];

      return makeRamp(["get", "speed"], limits, colorScale);
    } else {
      // Not visible
      return "red";
    }
  }

  function showLayer(style: RoadStyle): boolean {
    if (style == "cn") {
      // TODO another var from the other place
      return true;
    } else if (style == "existing_infra") {
      // TODO another var from the other place
      return true;
    } else if (style == "traffic") {
      // TODO another var from the other place
      return true;
    } else if (style == "gradient") {
      return true;
    } else if (style == "speed") {
      return true;
    } else {
      return false;
    }
  }
</script>

{#if $backend}
  {#await $backend.renderStaticRoads() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("static-roads")}
        filter={makeFilter($roadStyle)}
        paint={{
          "line-color": lineColor($roadStyle),
          "line-width": roadLineWidth(0),
        }}
        layout={{
          visibility: showLayer($roadStyle) ? "visible" : "none",
        }}
        manageHoverState
        hoverCursor="pointer"
      >
        <Popup openOn="click" let:props>
          <p>Traffic: {props.traffic.toLocaleString()}</p>
          <p>Gradient: {props.gradient.toFixed(1)}%</p>
          <p>Speed: {props.speed} mph</p>
          {#if props.cn}
            <p>Core network tier: {props.cn}</p>
          {/if}
          {#if props.existing_infra}
            <p>{infraTypeMapping[props.existing_infra][0]}</p>
          {/if}
          <a href={props.way} target="_blank">Open OSM</a>
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
