<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import {
    backend,
    mode,
    autosave,
    tier as currentTier,
    routeA,
    routeB,
    type RouteProps,
  } from "./stores";
  import type { FeatureCollection, LineString } from "geojson";
  import { onMount } from "svelte";
  import { colorByInfraType } from "./colors";
  import RouteControls from "./snapper/RouteControls.svelte";
  import { routeTool, waypoints, type Waypoint } from "./snapper/stores";
  import type { Map } from "maplibre-gl";
  import { notNull } from "svelte-utils";
  import { emptyGeojson, Popup } from "svelte-utils/map";

  export let map: Map;
  export let id: number | null;

  let name = "";
  let notes = "";
  let infraType = "Unknown";
  let tier = $currentTier;

  let existingGj: FeatureCollection<LineString, RouteProps> | null = null;

  let sectionsGj = emptyGeojson();
  $: recalculateSections($waypoints);

  onMount(async () => {
    existingGj = await $backend!.renderRoutes();

    $waypoints = [];
    if (id != null) {
      let feature = existingGj.features.find((f) => f.id == id)!;
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
        full_path: feature.properties.full_path,
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

  async function recalculateSections(waypts: Waypoint[]) {
    try {
      // TODO Wasteful; should RouteControls export a read-only view of this?
      let feature = JSON.parse($routeTool!.inner.calculateRoute(waypts));
      sectionsGj = await $backend!.autosplitRoute(feature.properties.full_path);
    } catch (err) {
      sectionsGj = emptyGeojson();
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
  </div>

  <span slot="extra-map">
    {#if existingGj}
      <GeoJSON data={existingGj}>
        <LineLayer
          filter={id == null ? undefined : ["!=", ["id"], id]}
          paint={{
            "line-width": 5,
            "line-color": colorByInfraType,
            "line-opacity": 0.5,
          }}
        />
      </GeoJSON>
    {/if}

    <GeoJSON data={sectionsGj}>
      <LineLayer
        filter={["==", ["get", "kind"], "new"]}
        paint={{
          "line-width": 3,
          "line-color": colorByInfraType,
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

    <!--<PickInfraType bind:current={infraType} />-->
    <label>
      Notes:
      <textarea rows="5" bind:value={notes} />
    </label>

    {#if sectionsGj.features.length > 0}
      <p>The route you've drawn has been split into sections:</p>
      <ol>
        {#each sectionsGj.features as f}
          {#if notNull(f.properties).kind == "new"}
            <li>
              A section where {notNull(f.properties).infra_type} is most appropriate
            </li>
          {:else}
            <li>An existing section from another route</li>
          {/if}
        {/each}
      </ol>
    {/if}
  </div>
</RouteControls>
