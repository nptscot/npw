<script lang="ts">
  import type { DataDrivenPropertyValueSpecification } from "maplibre-gl";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { constructMatchExpression } from "svelte-utils/map";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
    stageColors,
    tierColors,
    tierLabels,
  } from "../colors";
  import { layerId } from "../common";
  import { SplitComponent } from "../common/layout";
  import RelevantLayers from "../layers/RelevantLayers.svelte";
  import Greenspaces from "../local_access/Greenspaces.svelte";
  import PointPOIs from "../local_access/PointPOIs.svelte";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import { currentStage, editModeBreakdown, mode } from "../stores";
  import type { AutosplitRoute } from "../types";
  import AllSections from "./AllSections.svelte";

  export let ids: number[];
  export let sectionsGj: AutosplitRoute;

  let tier = $currentStage == "assessment" ? "Primary" : $currentStage;

  $: headerLabel = { ...tierLabels, assessment: "Assess" }[$currentStage];

  $: labelColor = stageColors[$currentStage];

  let colorSections = {
    infra_type: constructMatchExpression(
      ["get", "infra_type"],
      infraTypeColors,
      "black",
    ),
    gradient: constructMatchExpression(
      ["get", "gradient_group"],
      gradientColors,
      "black",
    ),
    deliverability: [
      "case",
      ["get", "fits"],
      "green",
      "red",
    ] as DataDrivenPropertyValueSpecification<string>,
    los: constructMatchExpression(
      ["get", "los"],
      levelOfServiceColors,
      "black",
    ),
    tier: constructMatchExpression(["get", "tier"], tierColors, "black"),
  };
</script>

<SplitComponent>
  <div slot="controls" class="left">
    <div class="main-controls">
      <header class="ds_page-header">
        <span
          class="ds_page-header__label ds_content-label"
          style:color={labelColor}
        >
          {headerLabel}
        </span>

        <h2 class="ds_page-header__title">Review route sections</h2>
      </header>

      <!-- TODO no BackLink? -->

      <button class="ds_button" on:click={() => ($mode = { kind: "main" })}>
        Continue
      </button>

      <p>This route was split into {ids.length} sections.</p>

      <AllSections {sectionsGj} {tier} />

      <RelevantLayers />
    </div>

    <LeftSidebarStats />
  </div>

  <div slot="map">
    <GeoJSON data={sectionsGj}>
      <LineLayer
        {...layerId("edit-route-sections")}
        paint={{
          "line-width": 10,
          "line-color": colorSections[$editModeBreakdown],
        }}
      />
    </GeoJSON>

    {#if $currentStage == "LocalAccess"}
      <Greenspaces />
      <PointPOIs />
    {/if}
  </div>
</SplitComponent>

<style>
  /** Controls **/
  .left {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
