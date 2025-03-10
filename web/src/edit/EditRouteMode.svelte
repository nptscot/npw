<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
    Map,
  } from "maplibre-gl";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Modal } from "svelte-utils";
  import { constructMatchExpression, emptyGeojson } from "svelte-utils/map";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
  } from "../colors";
  import { layerId, percent } from "../common";
  import AllControls from "../layers/AllControls.svelte";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import {
    autosave,
    backend,
    currentStage,
    mode,
    routeA,
    routeB,
  } from "../stores";
  import type { AutosplitRoute, Waypoint } from "../types";
  import PickInfraType from "./PickInfraType.svelte";
  import RouteControls from "./RouteControls.svelte";
  import SectionDiagram from "./SectionDiagram.svelte";
  import { waypoints } from "./stores";

  export let map: Map;
  export let id: number | null;

  let name = "";
  let notes = "";
  // This is not meaningful when overrideInfraType is false
  let infraType = "MixedTraffic";
  let overrideInfraType = false;
  let tier = $currentStage == "assessment" ? "Primary" : $currentStage;

  let showOverrideModal = false;

  let breakdown: "infra_type" | "gradient" | "deliverability" | "los" =
    "infra_type";

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

      // Transform into the correct format
      $waypoints = feature.properties.waypoints.map((waypt) => {
        return {
          point: [waypt.lon, waypt.lat],
          snapped: waypt.snapped,
        };
      });

      // TODO Debugging cases where auto-imported routes act oddly
      if (false) {
        let waypts1 = JSON.parse(JSON.stringify(feature.properties.waypoints));
        let full_path1 = JSON.parse(
          JSON.stringify(feature.properties.full_path),
        );
        let output = await $backend!.snapRoute($waypoints);
        let waypts2 = JSON.parse(JSON.stringify(output.properties.waypoints));
        let full_path2 = JSON.parse(
          JSON.stringify(output.properties.full_path),
        );

        if (JSON.stringify(waypts1) != JSON.stringify(waypts2)) {
          console.log(`waypts changed`);
          console.log(waypts1);
          console.log(waypts2);
        }
        if (JSON.stringify(full_path1) != JSON.stringify(full_path2)) {
          console.log(`full_path changed`);
          console.log(full_path1);
          console.log(full_path2);
        }
      }
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
    try {
      let feature = await $backend!.snapRoute($waypoints);
      // TODO Is this possible still?
      if (!feature) {
        window.alert("No route drawn");
        return;
      }

      await $backend!.setRoute(id, {
        feature,
        name,
        notes,
        full_path: feature.properties.full_path,
        infra_type: infraType,
        override_infra_type: overrideInfraType,
        tier,
      });
      await autosave();
      $mode = { kind: "main" };
    } catch (err) {
      window.alert(err);
    }
  }

  function cancel() {
    $mode = { kind: "main" };
  }

  // TODO Unless we recalculate immediately, this will be very misleading!
  async function evalRoute() {
    let feature = await $backend!.snapRoute($waypoints);
    let pt1 = feature.geometry.coordinates[0];
    let pt2 =
      feature.geometry.coordinates[feature.geometry.coordinates.length - 1];
    $routeA = { lng: pt1[0], lat: pt1[1] };
    $routeB = { lng: pt2[0], lat: pt2[1] };
    $mode = {
      kind: "evaluate-journey",
      prevMode: { kind: "edit-route", id },
      browse: [],
    };
  }

  async function recalculateSections(
    waypts: Waypoint[],
    overrideInfraType: boolean,
    infraType: string,
  ) {
    sectionsGj = emptyGeojson() as AutosplitRoute;

    try {
      // TODO Wasteful; should RouteControls export a read-only view of this?
      let feature = await $backend!.snapRoute(waypts);
      sectionsGj = await $backend!.autosplitRoute(
        id,
        feature.properties.full_path,
        overrideInfraType ? infraType : null,
      );
    } catch (err) {}
  }

  function percentFits(sectionsGj: AutosplitRoute): string {
    let total = 0;
    let fits = 0;
    for (let f of sectionsGj.features) {
      total += f.properties.length;
      if (f.properties.fits) {
        fits += f.properties.length;
      }
    }
    return percent(fits, total);
  }

  function percentHighLoS(sectionsGj: AutosplitRoute): string {
    let total = 0;
    let high = 0;
    for (let f of sectionsGj.features) {
      total += f.properties.length;
      if (f.properties.los == "High") {
        high += f.properties.length;
      }
    }
    return percent(high, total);
  }

  let filterSections = {
    infra_type: ["==", ["get", "kind"], "new"] as ExpressionSpecification,
    gradient: undefined,
    deliverability: undefined,
    los: undefined,
  };

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
  };
</script>

<RouteControls
  {map}
  {finish}
  {cancel}
  {deleteRoute}
  editingExisting={id != null}
>
  <div slot="extra-left">
    <label>
      Name:
      <input type="text" bind:value={name} />
    </label>

    {#if $waypoints.length >= 2}
      {#if overrideInfraType}
        <p>
          You've forced this route to always use {infraType}, assuming high
          Level of Service.
        </p>
        <button on:click={() => (overrideInfraType = false)}>
          Remove override
        </button>
      {:else}
        <p>
          The route you've drawn has been split into sections, automatically
          picking an infrastructure type to achieve the best possible Level of
          Service.
        </p>
        <button
          on:click={() => {
            overrideInfraType = true;
            showOverrideModal = true;
          }}
        >
          Override infrastructure type
        </button>
      {/if}

      {@const pctFits = percentFits(sectionsGj)}
      {#if pctFits != "100%"}
        <p>
          Only {pctFits} of the route fits in the available streetspace. You may
          need to override the infrastructure type for some sections.
        </p>
      {/if}

      {@const pctHighLoS = percentHighLoS(sectionsGj)}
      {#if pctHighLoS != "100%"}
        <p>
          Only {pctHighLoS} of the route has a high level of service. You may need
          to override the infrastructure type for some sections and reduce traffic
          speeds and volumes.
        </p>
      {/if}

      <label>
        Show details along route
        <select bind:value={breakdown}>
          <option value="infra_type">Infrastructure type</option>
          <option value="gradient">Gradient</option>
          <option value="deliverability">Streetspace deliverability</option>
          <option value="los">Level of Service</option>
        </select>
      </label>

      <SectionDiagram {breakdown} {sectionsGj} />
    {/if}

    <label>
      Notes:
      <textarea rows="5" bind:value={notes} />
    </label>

    <label>
      Tier:
      <select bind:value={tier}>
        <option value="Primary">Primary routes</option>
        <option value="Secondary">Secondary routes</option>
        <option value="LocalAccess">Local access routes</option>
        <option value="LongDistance">Long distance routes</option>
      </select>
    </label>

    <button
      class="secondary"
      on:click={evalRoute}
      disabled={$waypoints.length < 2}
    >
      Evaluate this route
    </button>

    <AllControls />

    <LeftSidebarStats />
  </div>

  <span slot="extra-map">
    <GeoJSON data={sectionsGj}>
      <LineLayer
        {...layerId("edit-route-sections")}
        filter={filterSections[breakdown]}
        paint={{
          "line-width": 10,
          "line-color": colorSections[breakdown],
        }}
      />
    </GeoJSON>
  </span>
</RouteControls>

<span class="pico">
  <Modal bind:show={showOverrideModal}>
    <PickInfraType bind:current={infraType} />
    <button on:click={() => (showOverrideModal = false)}>OK</button>
  </Modal>
</span>
