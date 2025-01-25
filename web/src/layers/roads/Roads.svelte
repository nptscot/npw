<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import {
    GeoJSON,
    JoinedData,
    LineLayer,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { notNull } from "svelte-utils";
  import { constructMatchExpression, makeRamp, Popup } from "svelte-utils/map";
  import {
    infraTypeColors,
    levelOfServiceColors,
    tierColors,
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
  import { lineColorForGradient } from "../../utils";
  import { severances } from "../stores";

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
    } else if (style == "current_infra") {
      return constructMatchExpression(
        ["feature-state", "current_infra"],
        infraTypeColors,
        "black",
      );
    } else if (style == "current_tier") {
      return constructMatchExpression(
        ["feature-state", "current_tier"],
        tierColors,
        "black",
      );
    } else if (style == "cn") {
      return constructMatchExpression(["get", "cn"], tierColors, "cyan");
    } else if (style == "existing_infra") {
      return constructMatchExpression(
        ["get", "existing_infra"],
        infraTypeColors,
        "black",
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
    } else if (style == "los") {
      return constructMatchExpression(
        ["feature-state", "los"],
        levelOfServiceColors,
        "black",
      );
    } else if (style == "reachability") {
      // TODO Copied
      let colors = {
        network: "green",
        reachable: "purple",
        severance: "red",
      };

      return constructMatchExpression(
        ["feature-state", "reachable"],
        colors,
        "black",
      );
    } else {
      // Not visible
      return "black";
    }
  }

  function showLayer(severances: boolean, style: RoadStyle): boolean {
    if (severances) {
      return true;
    } else if (style == "current_tier" || style == "current_infra") {
      return true;
    } else if (style == "cn") {
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
    } else if (style == "los") {
      // TODO another var from the other place
      return true;
    } else if (style == "reachability") {
      return true;
    } else {
      return false;
    }
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
          <Popup openOn="hover" let:props>
            {#if dynamicData[props.id].current_route_id != null}
              Edit {dynamicData[props.id].current_route_name}
            {/if}
          </Popup>
        {:else}
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
        {/if}
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
