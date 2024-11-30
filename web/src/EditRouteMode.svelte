<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { backend, mode, infraTypes, autosave } from "./stores";
  import type { Feature, FeatureCollection, LineString } from "geojson";
  import { onMount } from "svelte";
  import { colorByInfraType } from "./common";
  import RouteControls from "./snapper/RouteControls.svelte";
  import { routeTool, waypoints } from "./snapper/stores";
  import type { Map } from "maplibre-gl";

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
    }
  >;

  let name = "";
  let notes = "";
  let infraType = "Unknown";

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
      });
      await autosave();
    } catch (err) {
      window.alert(err);
    }
    $mode = { kind: "main" };
  }

  function cancel() {
    $mode = { kind: "main" };
  }
</script>

<RouteControls {map} {finish} {cancel} {deleteRoute}>
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

    <label>
      Infrastructure type:
      <select bind:value={infraType}>
        {#each infraTypes as [value, label, _]}
          <option {value}>{label}</option>
        {/each}
      </select>
    </label>

    <label>
      Notes:
      <textarea rows="5" bind:value={notes} />
    </label>
  </div>
</RouteControls>
