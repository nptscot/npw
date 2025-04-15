<script lang="ts">
  import type { DataDrivenPropertyValueSpecification, Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { constructMatchExpression, emptyGeojson } from "svelte-utils/map";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
    tierColors,
  } from "../colors";
  import { layerId } from "../common";
  import RelevantLayers from "../layers/RelevantLayers.svelte";
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
  import { waypoints } from "./stores";

  export let map: Map;
  export let id: number | null;

  let routeControls: RouteControls | null = null;

  let name = "";
  let notes = "";
  // This is not meaningful when overrideInfraType is false
  let infraType = "MixedTraffic";
  let overrideInfraType = false;
  let tier = $currentStage == "assessment" ? "Primary" : $currentStage;

  let sectionsGj: AutosplitRoute = emptyGeojson() as AutosplitRoute;
  $: recalculateSections($waypoints, overrideInfraType, infraType);

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
      infra_type: infraType as InfraType,
      override_infra_type: overrideInfraType,
      tier,
    });
    await autosave();
    $mode = { kind: "main" };
  }

  function cancel() {
    $mode = { kind: "main" };
  }

  async function recalculateSections(
    waypts: Waypoint[],
    overrideInfraType: boolean,
    infraType: string,
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

<RouteControls
  bind:this={routeControls}
  {map}
  {finish}
  {cancel}
  {deleteRoute}
  editingExisting={id != null}
  {tier}
>
  <div slot="extra-controls">
    {#if $waypoints.length >= 2}
      <AllSections {sectionsGj} bind:infraType bind:overrideInfraType {tier} />

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

  <svelte:fragment slot="sticky-bottom-controls">
    <LeftSidebarStats />
  </svelte:fragment>

  <span slot="extra-map">
    <GeoJSON data={sectionsGj}>
      <LineLayer
        {...layerId("edit-route-sections")}
        paint={{
          "line-width": 10,
          "line-color": colorSections[$editModeBreakdown],
        }}
      />
    </GeoJSON>
  </span>
</RouteControls>
