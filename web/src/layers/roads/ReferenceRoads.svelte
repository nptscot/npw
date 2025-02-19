<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import { LineLayer } from "svelte-maplibre";
  import { notNull } from "svelte-utils";
  import { constructMatchExpression, makeRamp, Popup } from "svelte-utils/map";
  import {
    cnTierColors,
    gradient,
    infraTypeColors,
    levelOfServiceColors,
    reachabilityColors,
    speed,
    traffic,
  } from "../../colors";
  import { layerId, Link, roadLineWidth } from "../../common";
  import {
    mode,
    referenceRoadStyle,
    type Mode,
    type ReferenceRoadStyle,
  } from "../../stores";
  import { infraTypeMapping, type DynamicRoad } from "../../types";
  import { debugOriginalData, severances } from "../stores";

  export let dynamicData: DynamicRoad[];

  function makeFilter(
    severances: boolean,
    style: ReferenceRoadStyle,
  ): ExpressionSpecification | undefined {
    if (severances) {
      return undefined;
    }
    if (style == "cn") {
      return ["to-boolean", ["get", "cn"]];
    } else if (style == "existing_infra") {
      return ["to-boolean", ["get", "existing_infra"]];
    }
    return undefined;
  }

  // TODO Filter doesn't work on feature-state
  function lineOpacity(
    mode: Mode,
    severances: boolean,
    style: ReferenceRoadStyle,
  ): DataDrivenPropertyValueSpecification<number> {
    let show = $mode.kind == "main" ? 1.0 : 0.5;

    if (severances) {
      return [
        "case",
        ["==", ["feature-state", "reachable"], "severance"],
        show,
        0.0,
      ];
    } else if (style == "reachability") {
      return [
        "case",
        ["==", ["feature-state", "reachable"], "unreachable"],
        0.0,
        show,
      ];
    }

    return show;
  }

  function lineColor(
    severances: boolean,
    style: ReferenceRoadStyle,
  ): DataDrivenPropertyValueSpecification<string> {
    if (severances) {
      return "red";
    }
    let invisibile = "black";
    return {
      off: invisibile,
      cn: constructMatchExpression(["get", "cn"], cnTierColors, "cyan"),
      existing_infra: constructMatchExpression(
        ["get", "existing_infra"],
        infraTypeColors,
        "black",
      ),
      traffic: makeRamp(["get", "traffic"], traffic.limits, traffic.colorScale),
      gradient: makeRamp(
        ["abs", ["get", "gradient"]],
        gradient.limits,
        gradient.colorScale,
      ),
      // TODO
      street_space: invisibile,
      speed: makeRamp(["get", "speed"], speed.limits, speed.colorScale),
      los: constructMatchExpression(
        ["feature-state", "los"],
        levelOfServiceColors,
        "black",
      ),
      reachability: constructMatchExpression(
        ["feature-state", "reachable"],
        reachabilityColors,
        "black",
      ),
      disconnections: invisibile,
      precalculated_rnet: invisibile,
      calculated_rnet: invisibile,
    }[style];
  }

  function showLayer(
    severances: boolean,
    debugOriginalData: boolean,
    style: ReferenceRoadStyle,
  ): boolean {
    if (severances) {
      return true;
    }
    if (
      debugOriginalData &&
      ["traffic", "los", "existing_infra", "speed", "cn"].includes(style)
    ) {
      return false;
    }
    return {
      off: false,
      cn: true,
      existing_infra: true,
      traffic: true,
      gradient: true,
      // TODO
      street_space: false,
      speed: true,
      los: true,
      reachability: true,
      disconnections: false,
      precalculated_rnet: false,
      calculated_rnet: false,
    }[style];
  }
</script>

<LineLayer
  {...layerId("reference-roads")}
  filter={makeFilter($severances, $referenceRoadStyle)}
  paint={{
    "line-color": lineColor($severances, $referenceRoadStyle),
    "line-opacity": lineOpacity($mode, $severances, $referenceRoadStyle),
    "line-width": roadLineWidth(0),
  }}
  layout={{
    visibility: showLayer($severances, $debugOriginalData, $referenceRoadStyle)
      ? "visible"
      : "none",
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
    <p>
      Precalculated cycling flow: {props.precalculated_flow.toLocaleString()} (quintile
      {props.precalculated_flow_quintile})
    </p>
    <a href={props.way} target="_blank">Open OSM</a>

    <hr />

    {#if dynamicData[props.id].current_route_id != null}
      {#if $mode.kind == "main"}
        <Link
          on:click={() => {
            $mode = {
              kind: "edit-route",
              id: dynamicData[props.id].current_route_id,
            };
          }}
        >
          Edit route
        </Link>
      {/if}

      <p>
        Part of {dynamicData[props.id].current_route_name}: {infraTypeMapping[
          notNull(dynamicData[props.id].current_infra)
        ][0]}
        ({dynamicData[props.id].current_tier} tier)
      </p>
    {/if}

    <p>Level of service: {dynamicData[props.id].los}</p>
    <p>Reachability: {dynamicData[props.id].reachable}</p>
  </Popup>
</LineLayer>
