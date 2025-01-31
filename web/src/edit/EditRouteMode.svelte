<script lang="ts">
  import type { Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Modal } from "svelte-utils";
  import {
    constructMatchExpression,
    emptyGeojson,
    Popup,
  } from "svelte-utils/map";
  import { infraTypeColors } from "../colors";
  import { layerId } from "../common";
  import AllControls from "../layers/AllControls.svelte";
  import PickEditsStyle from "../layers/roads/PickEditsStyle.svelte";
  import {
    autosave,
    backend,
    tier as currentTier,
    mode,
    routeA,
    routeB,
  } from "../stores";
  import type { AutosplitRoute } from "../types";
  import PickInfraType from "./PickInfraType.svelte";
  import RouteControls from "./RouteControls.svelte";
  import SectionDiagram from "./SectionDiagram.svelte";
  import { routeTool, waypoints, type Waypoint } from "./stores";

  export let map: Map;
  export let id: number | null;

  let name = "";
  let notes = "";
  // This is not meaningful when overrideInfraType is false
  let infraType = "MixedTraffic";
  let overrideInfraType = false;
  let tier = $currentTier;

  let showOverrideModal = false;

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
        let output = JSON.parse($routeTool!.inner.calculateRoute($waypoints));
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
      await $backend!.deleteRoute(id);
      await autosave();
    }
    $mode = { kind: "main" };
  }

  async function finish() {
    try {
      let feature = JSON.parse($routeTool!.inner.calculateRoute($waypoints));
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
  function evalRoute() {
    let feature = JSON.parse($routeTool!.inner.calculateRoute($waypoints));
    let pt1 = feature.geometry.coordinates[0];
    let pt2 =
      feature.geometry.coordinates[feature.geometry.coordinates.length - 1];
    $routeA = { lng: pt1[0], lat: pt1[1] };
    $routeB = { lng: pt2[0], lat: pt2[1] };
    $mode = {
      kind: "evaluate-route",
      prevMode: { kind: "edit-route", id },
      browse: [],
    };
  }

  async function recalculateSections(
    waypts: Waypoint[],
    overrideInfraType: boolean,
    infraType: string,
  ) {
    try {
      // TODO Wasteful; should RouteControls export a read-only view of this?
      let feature = JSON.parse($routeTool!.inner.calculateRoute(waypts));
      sectionsGj = await $backend!.autosplitRoute(
        id,
        feature.properties.full_path,
        overrideInfraType ? infraType : null,
      );
    } catch (err) {
      sectionsGj = emptyGeojson() as AutosplitRoute;
    }
  }
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

    <PickEditsStyle />
    <AllControls />
  </div>

  <span slot="extra-map">
    <GeoJSON data={sectionsGj}>
      <LineLayer
        {...layerId("edit-route-sections")}
        filter={["==", ["get", "kind"], "new"]}
        paint={{
          "line-width": 3,
          "line-color": constructMatchExpression(
            ["get", "infra_type"],
            infraTypeColors,
            "black",
          ),
        }}
      >
        <Popup openOn="hover" let:props>
          <p>This section will be {props.infra_type}</p>
        </Popup>
      </LineLayer>
    </GeoJSON>
  </span>

  <div slot="extra-right">
    <label>
      Name:
      <input type="text" bind:value={name} />
    </label>

    <label>
      Notes:
      <textarea rows="5" bind:value={notes} />
    </label>

    {#if sectionsGj.features.length > 0}
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
          picking an infrastructure type to achieve high Level of Service.
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

      <br />

      <SectionDiagram {sectionsGj} />
    {/if}
  </div>
</RouteControls>

{#if showOverrideModal}
  <span class="pico">
    <Modal on:close={() => (showOverrideModal = false)}>
      <PickInfraType bind:current={infraType} />
      <button on:click={() => (showOverrideModal = false)}>OK</button>
    </Modal>
  </span>
{/if}
