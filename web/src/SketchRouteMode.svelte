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

    if (id == null) {
      $routeTool!.startRoute();

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
    } else {
      let originalFeature = existingGj.features.find(
        (f) => f.id == id,
      )! as Feature<LineString, RouteProps & { name: string; notes: string }>;
      $routeTool!.editExistingRoute(originalFeature);

      $routeTool!.addEventListenerSuccess((f) => {
        let editedFeature = f as Feature<LineString, RouteProps>;
        try {
          $backend!.editRoute(id, {
            feature: editedFeature,
            name: originalFeature.properties.name,
            notes: originalFeature.properties.notes,
            nodes: editedFeature.properties.full_path,
          });
          $mode = {
            kind: "route-details",
            id,
          };
        } catch (err) {
          window.alert(err);
          $mode = {
            kind: "route-details",
            id,
          };
        }
      });
      $routeTool!.addEventListenerFailure(() => {
        $mode = {
          kind: "main",
        };
      });
    }
  });

  // The user can change the mode in many ways, like clicking a link.
  // When this component gets destroyed, always clean up state.
  onDestroy(() => {
    $routeTool?.clearEventListeners();
    $routeTool?.stop();
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
          filter={id == null ? undefined : ["!=", ["id"], id]}
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
