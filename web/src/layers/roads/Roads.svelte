<script lang="ts">
  import type { Feature } from "geojson";
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import {
    GeoJSON,
    JoinedData,
    LineLayer,
    Popup,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { notNull } from "svelte-utils";
  import {
    constructMatchExpression,
    makeRamp,
    Popup as WrappedPopup,
  } from "svelte-utils/map";
  import {
    cnTierColors,
    gradient,
    infraTypeColors,
    levelOfServiceColors,
    reachabilityColors,
    speed,
    tierColors,
    traffic,
  } from "../../colors";
  import { layerId, Link, roadLineWidth } from "../../common";
  import {
    backend,
    mode,
    mutationCounter,
    roadStyle,
    type Mode,
    type RoadStyle,
  } from "../../stores";
  import { infraTypeMapping, type DynamicRoad } from "../../types";
  import { severances } from "../stores";
  import CyclingFlowCoverage from "./CyclingFlowCoverage.svelte";

  let lastUpdate = 0;
  // The Popup code assumes that 'props.id' indexes into this
  // Array<Record<string, string | number | undefined>>
  let dynamicData: DynamicRoad[] = [];

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      dynamicData = await $backend.renderDynamicRoads();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($mutationCounter > 0) {
    recalc();
  }

  $: clickToEdit =
    $mode.kind == "main" &&
    ($roadStyle == "current_infra" || $roadStyle == "current_tier");

  function editRouteMap(e: CustomEvent<LayerClickInfo>) {
    if (clickToEdit) {
      let road_id = e.detail.features[0].id as number;
      let route_id = dynamicData[road_id].current_route_id;
      // If it's null, we clicked an opacity 0 road that's not part of a route
      if (route_id != null) {
        $mode = { kind: "edit-route", id: route_id };
      }
    }
  }

  function makeFilter(style: RoadStyle): ExpressionSpecification | undefined {
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
    style: RoadStyle,
  ): DataDrivenPropertyValueSpecification<number> {
    let show = $mode.kind == "main" ? 1.0 : 0.5;

    if (severances) {
      return [
        "case",
        ["==", ["feature-state", "reachable"], "severance"],
        show,
        0.0,
      ];
    } else if (style == "current_infra" || style == "current_tier") {
      // While editing an existing route, hide it
      if ($mode.kind == "edit-route" && $mode.id != null) {
        return [
          "case",
          [
            "all",
            ["to-boolean", ["feature-state", "current_infra"]],
            ["!=", $mode.id, ["feature-state", "current_route_id"]],
          ],
          show,
          0.0,
        ];
      }

      return [
        "case",
        ["to-boolean", ["feature-state", "current_infra"]],
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
    style: RoadStyle,
  ): DataDrivenPropertyValueSpecification<string> {
    if (severances) {
      return "red";
    }
    let invisibile = "black";
    return {
      off: invisibile,
      current_infra: constructMatchExpression(
        ["feature-state", "current_infra"],
        infraTypeColors,
        "black",
      ),
      current_tier: constructMatchExpression(
        ["feature-state", "current_tier"],
        tierColors,
        "black",
      ),
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

  function showLayer(severances: boolean, style: RoadStyle): boolean {
    if (severances) {
      return true;
    }
    return {
      off: false,
      current_infra: true,
      current_tier: true,
      cn: true,
      existing_infra: true,
      traffic: true,
      gradient: true,
      speed: true,
      los: true,
      reachability: true,
      disconnections: false,
      precalculated_rnet: false,
      calculated_rnet: false,
    }[style];
  }

  function showEditPopup(features: Feature[]): boolean {
    let roadId = features[0]?.properties?.id;
    return dynamicData[roadId].current_route_id != null;
  }
</script>

{#if $backend}
  {#await $backend.renderStaticRoads() then data}
    <GeoJSON {data} promoteId="id">
      <JoinedData data={dynamicData} idCol="id" />

      <LineLayer
        {...layerId("npw-roads")}
        filter={makeFilter($roadStyle)}
        paint={{
          "line-color": lineColor($severances, $roadStyle),
          "line-opacity": lineOpacity($mode, $severances, $roadStyle),
          "line-width": roadLineWidth(0),
        }}
        layout={{
          visibility: showLayer($severances, $roadStyle) ? "visible" : "none",
        }}
        manageHoverState
        hoverCursor="pointer"
        on:click={editRouteMap}
      >
        {#if clickToEdit}
          <Popup openOn="hover" let:data canOpen={showEditPopup}>
            {@const props = data?.properties}
            {#if props && dynamicData[props.id].current_route_id != null}
              Edit {dynamicData[props.id].current_route_name}
            {/if}
          </Popup>
        {:else}
          <WrappedPopup openOn="click" let:props>
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
              Precalculated cycling flow quintile: {props.precalculated_flow_quintile}
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
          </WrappedPopup>
        {/if}
      </LineLayer>

      <CyclingFlowCoverage />
    </GeoJSON>
  {/await}
{/if}
