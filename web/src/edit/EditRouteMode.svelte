<script lang="ts">
  import type { DataDrivenPropertyValueSpecification, Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { notNull } from "svelte-utils";
  import { constructMatchExpression, emptyGeojson } from "svelte-utils/map";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
    stageColors,
    tierColors,
    tierLabels,
  } from "../colors";
  import { BackLink, Checkbox, layerId } from "../common";
  import { SplitComponent } from "../common/layout";
  import RelevantLayers from "../layers/RelevantLayers.svelte";
  import { majorJunctions } from "../layers/stores";
  import Greenspaces from "../local_access/Greenspaces.svelte";
  import PointPOIs from "../local_access/PointPOIs.svelte";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import {
    autosave,
    backend,
    currentStage,
    editModeBreakdown,
    mode,
  } from "../stores";
  import type { AutosplitRoute, InfraType, Waypoint } from "../types";
  import AllSections from "./AllSections.svelte";
  import RouteControls from "./RouteControls.svelte";
  import { canStopDrawing, waypoints } from "./stores";

  export let map: Map;
  export let id: number | null;

  let routeControls: RouteControls | null = null;
  let cannotUndo = true;

  let name = "";
  let notes = "";
  // This is not meaningful when overrideInfraType is false
  let infraType: InfraType = "MixedTraffic";
  let overrideInfraType = false;
  let tier = $currentStage == "assessment" ? "Primary" : $currentStage;

  let sectionsGj: AutosplitRoute = emptyGeojson() as AutosplitRoute;
  $: recalculateSections($waypoints, overrideInfraType, infraType);

  $: headerLabel = { ...tierLabels, assessment: "Assess" }[$currentStage];

  $: labelColor = stageColors[$currentStage];

  onMount(async () => {
    $waypoints = [];
    if (id != null) {
      let feature = await $backend!.getRoute(id);
      name = feature.properties.name;
      notes = feature.properties.notes;
      infraType = feature.properties.infra_type;
      overrideInfraType = feature.properties.override_infra_type;
      tier = feature.properties.tier;

      $waypoints = feature.properties.waypoints;
    }
  });

  async function deleteRoute() {
    if (!window.confirm("Are you sure you want to delete this route?")) {
      return;
    }
    if (id != null) {
      await $backend!.deleteRoutes([id]);
      await autosave();
    }
    $mode = { kind: "main" };
  }

  async function finish() {
    await $backend!.setRoute(id, {
      waypoints: $waypoints,

      name,
      notes,
      infra_type: infraType,
      override_infra_type: overrideInfraType,
      tier,
    });
    await autosave();
    $mode = { kind: "main" };
  }

  function cancel() {
    $mode = { kind: "main" };
  }

  function maybeCancel() {
    if (canStopDrawing()) {
      cancel();
    }
  }

  async function recalculateSections(
    waypts: Waypoint[],
    overrideInfraType: boolean,
    infraType: InfraType,
  ) {
    try {
      sectionsGj = await $backend!.autosplitRoute(
        id,
        waypts,
        overrideInfraType ? infraType : null,
        tier,
        routeControls!.majorSnapThreshold(),
      );
    } catch (err) {}
  }

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

        {#if id != null}
          <h2 class="ds_page-header__title">Edit a route</h2>
        {:else}
          <h2 class="ds_page-header__title">Draw a new route</h2>
        {/if}
      </header>

      <BackLink on:click={maybeCancel}>Stop drawing</BackLink>

      {#if $currentStage == "Primary" || $currentStage == "Secondary"}
        <Checkbox bind:checked={$majorJunctions} small>
          Snap to main roads
        </Checkbox>
      {/if}

      {#if $waypoints.length == 0}
        <p>Click to set the start of the route.</p>
      {:else if $waypoints.length == 1}
        <p>Click to set the end of the route.</p>
      {:else}
        <p>
          Click to extend the route, drag points to adjust, or change the route
          properties.
        </p>

        <button class="ds_button" on:click={finish}>Finish</button>

        {#if id != null}
          <div>
            <button
              class="ds_button ds_button--secondary"
              on:click={deleteRoute}
            >
              Delete
            </button>
          </div>
        {/if}

        <div>
          <button
            class="ds_button ds_button--secondary"
            disabled={cannotUndo}
            on:click={notNull(routeControls).undo}
            title="Undo last change on the map"
          >
            Undo
          </button>
        </div>
      {/if}

      {#if $waypoints.length >= 2}
        <AllSections
          {sectionsGj}
          bind:infraType
          bind:overrideInfraType
          {tier}
        />

        <h4>Route properties</h4>

        <input
          class="ds_input ds_input--fixed-20"
          placeholder="Name"
          bind:value={name}
        />

        <textarea
          class="ds_input"
          rows="2"
          placeholder="Notes"
          bind:value={notes}
        />

        <div>
          <label>
            Tier:
            <select bind:value={tier}>
              <option value="Primary">Primary routes</option>
              <option value="Secondary">Secondary routes</option>
              <option value="LocalAccess">Local access routes</option>
              <option value="LongDistance">Long distance routes</option>
            </select>
          </label>
        </div>
      {/if}

      <RelevantLayers />
    </div>

    <LeftSidebarStats />
  </div>

  <div slot="map">
    <RouteControls
      bind:this={routeControls}
      bind:cannotUndo
      {map}
      {finish}
      {cancel}
      {tier}
    />

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
