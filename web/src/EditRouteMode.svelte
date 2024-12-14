<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import {
    backend,
    mode,
    autosave,
    tier as currentTier,
    type Tier,
    routeA,
    routeB,
  } from "./stores";
  import type { Feature, FeatureCollection, LineString } from "geojson";
  import { onMount } from "svelte";
  import { colorByInfraType } from "./colors";
  import RouteControls from "./snapper/RouteControls.svelte";
  import { routeTool, waypoints } from "./snapper/stores";
  import type { Map } from "maplibre-gl";
  import PickInfraType from "./PickInfraType.svelte";

  export let map: Map;
  export let id: number | null;

  // TODO Move to stores
  type RouteFeature = Feature<
    LineString,
    // TODO route props too
    {
      waypoints: any[];
      full_path: any[];
      name: string;
      notes: string;
      infra_type: string;
      tier: Tier;
    }
  >;

  let name = "";
  let notes = "";
  let infraType = "Unknown";
  let tier = $currentTier;

  let existingGj: FeatureCollection | null = null;

  onMount(async () => {
    existingGj = await $backend!.renderRoutes();

    $waypoints = [];
    if (id != null) {
      let feature = existingGj.features.find(
        (f) => f.id == id,
      )! as RouteFeature;
      name = feature.properties.name;
      notes = feature.properties.notes;
      infraType = feature.properties.infra_type;
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
    if (id) {
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
        nodes: feature.properties.full_path,
        infra_type: infraType,
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
  </div>

  <span slot="extra-map">
    {#if existingGj}
      <GeoJSON data={existingGj}>
        <LineLayer
          id="routes"
          filter={id == null ? undefined : ["!=", ["id"], id]}
          paint={{
            "line-width": 5,
            "line-color": colorByInfraType,
            "line-opacity": 0.5,
          }}
        />
      </GeoJSON>
    {/if}
  </span>

  <div slot="extra-right">
    <label>
      Name:
      <input type="text" bind:value={name} />
    </label>

    <PickInfraType bind:current={infraType} />

    <label>
      Notes:
      <textarea rows="5" bind:value={notes} />
    </label>
  </div>
</RouteControls>
