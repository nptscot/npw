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
  import { constructMatchExpression } from "svelte-utils/map";
  import {
    infraTypeColors,
    levelOfServiceColors,
    tierColors,
  } from "../../colors";
  import { layerId, roadLineWidth } from "../../common";
  import {
    backend,
    editsRoadStyle,
    mode,
    mutationCounter,
    type EditsRoadStyle,
    type Mode,
  } from "../../stores";
  import type { DynamicRoad, InfraType, Tier } from "../../types";
  import {
    showNetworkDeliverability,
    showNetworkInfraTypes,
    showNetworkLoS,
    showNetworkTiers,
  } from "../stores";
  import CyclingDemandCoverage from "./CyclingDemandCoverage.svelte";
  import MainRoadCoverage from "./MainRoadCoverage.svelte";
  import ReferenceRoads from "./ReferenceRoads.svelte";

  let lastUpdate = 0;
  // The Popup code assumes that 'props.id' indexes into this
  // Array<Record<string, string | number | undefined>>
  let dynamicData: DynamicRoad[] = [];

  let hovered: Feature | null;

  function castToRecord(
    d: DynamicRoad[],
  ): Record<string, string | number | undefined>[] {
    // @ts-expect-error TODO Change DynamicRoad definition and/or upstream to allow null as well
    return d as Record<string, string | number | undefined>[];
  }

  async function recalc() {
    if ($backend && lastUpdate != $mutationCounter) {
      dynamicData = await $backend.renderDynamicRoads();
      lastUpdate = $mutationCounter;
    }
  }

  $: if ($mutationCounter > 0) {
    recalc();
  }

  function editRouteMap(e: CustomEvent<LayerClickInfo>) {
    if ($mode.kind == "main") {
      let road_id = e.detail.features[0].id as number;
      let route_id = dynamicData[road_id].current_route_id;
      // If it's null, we clicked an opacity 0 road that's not part of a route
      if (route_id != null) {
        $mode = { kind: "edit-route", id: route_id };
      }
    }
  }

  // TODO Filter doesn't work on feature-state
  function lineOpacity(
    mode: Mode,
    style: EditsRoadStyle,
    showTiers: { [name in Tier]: boolean },
    showInfraTypes: { [name in InfraType]: boolean },
    showDeliverable: { [name: string]: boolean },
    showNetworkLoS: { [name: string]: boolean },
    hovered: Feature | null,
  ): DataDrivenPropertyValueSpecification<number> {
    // Moot point, invisibile anyway
    if (style == "off") {
      return 0.0;
    }

    // @ts-expect-error Guaranteed to be set below
    let showLayer: ExpressionSpecification = null;
    if (style == "edits_infra") {
      let include = Object.keys(showInfraTypes).filter(
        (k) => showInfraTypes[k as InfraType],
      );
      showLayer = [
        "in",
        ["feature-state", "current_infra"],
        ["literal", include],
      ];
    } else if (style == "edits_tier") {
      let include = Object.keys(showTiers).filter((k) => showTiers[k as Tier]);
      showLayer = [
        "in",
        ["feature-state", "current_tier"],
        ["literal", include],
      ];
    } else if (style == "edits_deliverability") {
      let include = [];
      if (showDeliverable.deliverable) {
        include.push(true);
      }
      if (showDeliverable.not) {
        include.push(false);
      }
      showLayer = [
        "all",
        ["to-boolean", ["feature-state", "current_infra"]],
        ["in", ["feature-state", "current_infra_fits"], ["literal", include]],
      ];
    } else if (style == "edits_los") {
      let include = Object.keys(showNetworkLoS).filter(
        (k) => showNetworkLoS[k],
      );
      showLayer = [
        "all",
        ["to-boolean", ["feature-state", "current_infra"]],
        ["in", ["feature-state", "los"], ["literal", include]],
      ];
    }

    // While editing an existing route, hide it
    if ($mode.kind == "edit-route" && $mode.id != null) {
      return [
        "case",
        [
          "all",
          ["to-boolean", ["feature-state", "current_infra"]],
          ["!=", $mode.id, ["feature-state", "current_route_id"]],
          showLayer,
        ],
        1.0,
        0.0,
      ];
    }

    // @ts-expect-error This really works
    let highlightHoveredRoute: ExpressionSpecification = 1.0;
    if (hovered != null) {
      let roadId = hovered.properties!.id;
      let routeId = dynamicData[roadId].current_route_id;
      if (routeId != null) {
        highlightHoveredRoute = [
          "case",
          ["==", ["feature-state", "current_route_id"], routeId],
          0.5,
          1.0,
        ];
      }
    }

    return ["case", showLayer, highlightHoveredRoute, 0.0];
  }

  function lineColor(
    style: EditsRoadStyle,
  ): DataDrivenPropertyValueSpecification<string> {
    let invisibile = "black";
    return {
      off: invisibile,
      edits_infra: constructMatchExpression(
        ["feature-state", "current_infra"],
        infraTypeColors,
        "black",
      ),
      edits_tier: constructMatchExpression(
        ["feature-state", "current_tier"],
        tierColors,
        "black",
      ),
      edits_deliverability: [
        "case",
        ["feature-state", "current_infra_fits"],
        "green",
        "red",
      ] as DataDrivenPropertyValueSpecification<string>,
      edits_los: constructMatchExpression(
        ["feature-state", "los"],
        levelOfServiceColors,
        "black",
      ),
    }[style];
  }

  function showEditPopup(features: Feature[]): boolean {
    if ($mode.kind != "main") {
      return false;
    }
    let roadId = features[0]?.properties?.id;
    return dynamicData[roadId].current_route_id != null;
  }
</script>

{#if $backend}
  {#await $backend.renderStaticRoads() then data}
    <GeoJSON {data} promoteId="id">
      <JoinedData data={castToRecord(dynamicData)} idCol="id" />

      <LineLayer
        {...layerId("edits-roads")}
        paint={{
          "line-color": lineColor($editsRoadStyle),
          "line-opacity": lineOpacity(
            $mode,
            $editsRoadStyle,
            $showNetworkTiers,
            $showNetworkInfraTypes,
            $showNetworkDeliverability,
            $showNetworkLoS,
            hovered,
          ),
          "line-width": roadLineWidth(1),
        }}
        layout={{
          visibility:
            $editsRoadStyle == "off" || $mode.kind == "bulk-edit"
              ? "none"
              : "visible",
        }}
        manageHoverState
        hoverCursor="pointer"
        on:click={editRouteMap}
        bind:hovered
      >
        <Popup openOn="hover" let:data canOpen={showEditPopup}>
          {@const props = data?.properties}
          {#if props && dynamicData[props.id].current_route_id != null}
            Edit {dynamicData[props.id].current_route_name}
          {/if}
        </Popup>
      </LineLayer>
      <ReferenceRoads {dynamicData} />

      <CyclingDemandCoverage />
      <MainRoadCoverage />
    </GeoJSON>
  {/await}
{/if}
