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
  import {
    layerId,
    lineColorForDemand,
    lineWidthForDemand,
    roadLineWidth,
  } from "../../common";
  import {
    backgroundLayer,
    mode,
    type BackgroundLayer,
    type Mode,
  } from "../../stores";
  import {
    infraTypeMapping,
    type DynamicRoad,
    type InfraType,
  } from "../../types";
  import {
    debugCyclingDemandMin,
    debugOriginalData,
    severances,
    styleCyclingDemand,
  } from "../stores";

  export let dynamicData: DynamicRoad[];

  function makeFilter(
    severances: boolean,
    debugCyclingDemandMin: number,
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
    } else if (style == "precalculated_rnet") {
      return [">=", ["get", "precalculated_demand"], debugCyclingDemandMin];
    }
    return undefined;
  }

  // TODO Filter doesn't work on feature-state
  function lineOpacity(
    mode: Mode,
    severances: boolean,
    style: BackgroundLayer,
  ): DataDrivenPropertyValueSpecification<number> {
    let show = $mode.kind == "edit-route" ? 0.5 : 1.0;

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
    styleCyclingDemand: boolean,
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
      street_space: [
        "case",
        ["get", "segregated_fits"],
        "green",
        "red",
      ] as DataDrivenPropertyValueSpecification<string>,
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
      precalculated_rnet: styleCyclingDemand
        ? lineColorForDemand(["get", "precalculated_demand"])
        : "grey",
      calculated_rnet: invisibile,
      population: invisibile,
      deprived: invisibile,
    }[style];
  }

  function lineWidth(
    styleCyclingDemand: boolean,
    style: BackgroundLayer,
  ): DataDrivenPropertyValueSpecification<number> {
    if (style == "precalculated_rnet") {
      return styleCyclingDemand
        ? lineWidthForDemand(["get", "precalculated_demand"])
        : roadLineWidth(4);
    }
    return roadLineWidth(0);
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
      precalculated_rnet: true,
      calculated_rnet: false,
      population: false,
      deprived: false,
    }[style];
  }

  function castInfraType(x: string): InfraType {
    return x as InfraType;
  }

  // Don't expose exact traffic counts
  function trafficGroup(x: number): string {
    if (x < 1000) {
      return "0 to 999";
    } else if (x < 2000) {
      return "1000 to 1999";
    } else if (x < 4000) {
      return "2000 to 3999";
    } else {
      return "4000+";
    }
  }
</script>

<LineLayer
  {...layerId("reference-roads")}
  filter={makeFilter($severances, $debugCyclingDemandMin, $backgroundLayer)}
  paint={{
    "line-color": lineColor($severances, $styleCyclingDemand, $backgroundLayer),
    "line-opacity": lineOpacity($mode, $severances, $backgroundLayer),
    "line-width": lineWidth($styleCyclingDemand, $backgroundLayer),
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
    <div style="max-width: 30vw; max-height: 60vh; overflow: auto;">
      <p>
        Arterial road? <b>{props.is_arterial_road ? "yes" : "no"}</b>
      </p>
      <p>
        Within a settlement? <b>{props.within_settlement ? "yes" : "no"}</b>
      </p>
      <p>
        Is next to greenspace? <b>{props.is_attractive ? "yes" : "no"}</b>
      </p>
      <p>
        Traffic: <b>{trafficGroup(props.traffic)}</b>
      </p>
      <p>
        Gradient: <b>{props.gradient.toFixed(1)}%</b>
      </p>
      <p>
        Speed: <b>{props.speed} mph</b>
      </p>
      {#if props.cn}
        <p>
          Coherent network tier: <b>{props.cn}</b>
        </p>
      {/if}
      {#if props.existing_infra}
        <p>
          Existing infrastructure: <b>
            {infraTypeMapping[castInfraType(props.existing_infra)][0]}
          </b>
        </p>
      {/if}
      <p>
        Precalculated cycling demand: <b>
          {props.precalculated_demand.toLocaleString()}
          (group
          {props.precalculated_demand_group})
        </b>
      </p>
      {#if props.street_space}
        {@const ss = JSON.parse(props.street_space)}
        <p>Streetspace evaluation data:</p>
        <ul>
          <li>
            Does a segregated cycletrack fit? <b>
              {ss.segregated_fits ? "yes" : "no"}
            </b>
          </li>
          <li>
            Edge-to-edge width: <b>{ss.edge_to_edge_width}m</b>
          </li>
          <li>
            Details: <b>{ss.cross_section_profile}</b>
          </li>
        </ul>
      {:else}
        <p>No streetspace evaluation data</p>
      {/if}
      <a href={props.way} target="_blank">Open OSM</a>

      <hr />

      {#if dynamicData[props.id].current_route_id != null}
        <p>
          Part of route <b>{dynamicData[props.id].current_route_name}</b>
          : {infraTypeMapping[notNull(dynamicData[props.id].current_infra)][0]}
          ({dynamicData[props.id].current_tier} tier)
        </p>

        <p>
          This infrastructure type <b>
            {dynamicData[props.id].current_infra_fits ? "does" : "does not"}
          </b>
          fit the available streetspace
        </p>
      {/if}

      <p>
        Level of service: <b>{dynamicData[props.id].los}</b>
      </p>
      <p>
        Reachability: <b>{dynamicData[props.id].reachable}</b>
      </p>
    </div>
  </Popup>
</LineLayer>
