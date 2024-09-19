<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { backend, mode, routeTool } from "./stores";
  import { onMount, onDestroy } from "svelte";
  import RouteSnapperLayer from "./snapper/RouteSnapperLayer.svelte";
  import RouteControls from "./snapper/RouteControls.svelte";
  import type { RouteProps } from "route-snapper-ts";
  import type { Feature, FeatureCollection, LineString } from "geojson";

  export let id: number | null;

  let existingGj: FeatureCollection | null = null;
  onMount(async () => {
    existingGj = await $backend!.renderRoutes();
  });

  // The user can change the mode in many ways, like clicking a link.
  // When this component gets destroyed, always clean up state.
  onDestroy(() => {
    $routeTool?.clearEventListeners();
    $routeTool?.stop();
  });

  $routeTool!.addEventListenerSuccess((f) => {
    let feature = f as Feature<LineString, RouteProps>;
    try {
      let newId = $backend!.newRoute({
        feature,
        name: "",
        notes: "",
        nodes: feature.properties.full_path,
      });
      $mode = {
        kind: "route-details",
        id: newId,
      };
    } catch (err) {
      window.alert(err);
      $mode = {
        kind: "main",
      };
    }
  });
  $routeTool!.addEventListenerFailure(() => {
    $mode = {
      kind: "main",
    };
  });
</script>

<SplitComponent>
  <div slot="top">Nav</div>
  <div slot="sidebar">
    <h2>Sketch route mode</h2>
    {#if $routeTool}
      <RouteControls routeTool={$routeTool} />
    {/if}
  </div>

  <div slot="map">
    {#if existingGj}
      <GeoJSON data={existingGj}>
        <LineLayer
          id="routes"
          paint={{
            "line-width": 5,
            "line-color": "red",
            "line-opacity": 0.5,
          }}
        />
      </GeoJSON>
    {/if}

    <RouteSnapperLayer />
  </div>
</SplitComponent>
