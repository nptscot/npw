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
    streetSpaceColors,
    traffic,
  } from "../../colors";
  import { layerId, roadLineWidth } from "../../common";
  import {
    backgroundLayer,
    mode,
    type BackgroundLayer,
    type Mode,
  } from "../../stores";
  import { infraTypeMapping, type DynamicRoad } from "../../types";
  import { debugOriginalData, severances } from "../stores";

  export let dynamicData: DynamicRoad[];

  function makeFilter(
    severances: boolean,
    style: BackgroundLayer,
  ): ExpressionSpecification | undefined {
    if (severances) {
      return undefined;
    }
    if (style == "cn") {
      return ["to-boolean", ["get", "cn"]];
    } else if (style == "existing_infra") {
      return ["to-boolean", ["get", "existing_infra"]];
    } else if (style == "street_space") {
      return ["to-boolean", ["get", "street_space"]];
    } else if (style == "attractive") {
      return ["get", "is_attractive"];
    }
    return undefined;
  }

  // TODO Filter doesn't work on feature-state
  function lineOpacity(
    mode: Mode,
    severances: boolean,
    style: BackgroundLayer,
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
    } else if (style == "deliverability") {
      return [
        "case",
        ["to-boolean", ["feature-state", "current_infra"]],
        show,
        0.0,
      ];
    }

    return show;
  }

  function lineColor(
    severances: boolean,
    style: BackgroundLayer,
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
      street_space: constructMatchExpression(
        ["get", "street_space"],
        streetSpaceColors,
        "cyan",
      ),
      speed: makeRamp(["get", "speed"], speed.limits, speed.colorScale),
      attractive: "green",
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
      deliverability: [
        "case",
        ["feature-state", "current_infra_fits"],
        "green",
        "red",
      ] as DataDrivenPropertyValueSpecification<string>,
      population: invisibile,
      deprived: invisibile,
    }[style];
  }

  function showLayer(
    severances: boolean,
    debugOriginalData: boolean,
    style: BackgroundLayer,
  ): boolean {
    if (severances) {
      return true;
    }
    if (
      debugOriginalData &&
      [
        "traffic",
        "los",
        "existing_infra",
        "speed",
        "cn",
        "street_space",
      ].includes(style)
    ) {
      return false;
    }
    return {
      off: false,
      cn: true,
      existing_infra: true,
      traffic: true,
      gradient: true,
      street_space: true,
      speed: true,
      attractive: true,
      los: true,
      reachability: true,
      disconnections: false,
      precalculated_rnet: false,
      calculated_rnet: false,
      deliverability: true,
      population: false,
      deprived: false,
    }[style];
  }
</script>

<LineLayer
  {...layerId("reference-roads")}
  filter={makeFilter($severances, $backgroundLayer)}
  paint={{
    "line-color": lineColor($severances, $backgroundLayer),
    "line-opacity": lineOpacity($mode, $severances, $backgroundLayer),
    "line-width": roadLineWidth(0),
  }}
  layout={{
    visibility: showLayer($severances, $debugOriginalData, $backgroundLayer)
      ? "visible"
      : "none",
  }}
  manageHoverState
  hoverCursor="pointer"
>
  <Popup openOn="click" let:props>
    <p>Main road? {props.is_main_road ? "yes" : "no"}</p>
    <p>Within a settlement? {props.within_settlement ? "yes" : "no"}</p>
    <p>Is next to greenspace? {props.is_attractive ? "yes" : "no"}</p>
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
      Precalculated cycling demand: {props.precalculated_demand.toLocaleString()}
      (group
      {props.precalculated_demand_group})
    </p>
    {#if props.street_space}
      <p>
        What fits within the carriageway, verges, and footways? {props.street_space}
      </p>
    {/if}
    <a href={props.way} target="_blank">Open OSM</a>

    <hr />

    {#if dynamicData[props.id].current_route_id != null}
      <p>
        Part of {dynamicData[props.id].current_route_name}: {infraTypeMapping[
          notNull(dynamicData[props.id].current_infra)
        ][0]}
        ({dynamicData[props.id].current_tier} tier)
      </p>

      <p>
        This infrastructure type {dynamicData[props.id].current_infra_fits
          ? "does"
          : "does not"} fit the available streetspace
      </p>
    {/if}

    <p>Level of service: {dynamicData[props.id].los}</p>
    <p>Reachability: {dynamicData[props.id].reachable}</p>
  </Popup>
</LineLayer>
