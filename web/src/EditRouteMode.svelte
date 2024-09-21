<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { backend, mode, infraTypes, autosave, routeTool } from "./stores";
  import type { Feature, FeatureCollection, LineString } from "geojson";
  import type { RouteProps } from "route-snapper-ts";
  import { onMount, onDestroy } from "svelte";
  import { colorByInraType } from "./common";
  import RouteSnapperLayer from "./snapper/RouteSnapperLayer.svelte";
  import RouteControls from "./snapper/RouteControls.svelte";

  export let id: number | null;

  // TODO Move to stores
  type RouteFeature = Feature<
    LineString,
    RouteProps & { name: string; notes: string; infra_type: string }
  >;

  let name = "";
  let notes = "";
  let infraType = "Unknown";

  let gj: FeatureCollection | null = null;
  let feature: RouteFeature | null = null;

  onMount(async () => {
    gj = await $backend!.renderRoutes();

    console.log(`starting up, id initially is ${id}`);
    if (id == null) {
      $routeTool!.startRoute();
    } else {
      feature = gj.features.find((f) => f.id == id)! as RouteFeature;
      name = feature.properties!.name;
      notes = feature.properties!.notes;
      infraType = feature.properties!.infra_type;

      $routeTool!.editExistingRoute(feature);
    }

    $routeTool!.addEventListenerSuccess(async (f) => {
      let editedFeature = f as RouteFeature;
      try {
        // TODO Simplify backend API
        if (id == null) {
          console.log("success. making new route");
          id = $backend!.newRoute({
            feature: editedFeature,
            name,
            notes,
            nodes: editedFeature.properties.full_path,
            infra_type: infraType,
          });
        } else {
          console.log("success. not editing this route yet");
        }
        feature = editedFeature;

        // Start editing again
        console.log("start editing again, after success");
        $routeTool!.editExistingRoute(feature!);
      } catch (err) {
        window.alert(err);
        // Start editing again
        if (feature) {
          $routeTool!.editExistingRoute(feature);
        } else {
          $routeTool!.startRoute();
        }
      }
    });

    $routeTool!.addEventListenerFailure(() => {
      console.log("failure. restarting");
      // Start editing again
      if (feature) {
        $routeTool!.editExistingRoute(feature);
      } else {
        $routeTool!.startRoute();
      }
    });
  });

  onDestroy(async () => {
    // Trigger saving the current thing
    console.log("component destroy");
    $routeTool?.finish();
    $routeTool?.clearEventListeners();
    $routeTool?.stop();

    if (id != null && feature) {
      console.log("in component destroy, do save this thing");
      // TODO This API is weird
      await $backend!.editRoute(id, {
        feature,
        name,
        notes,
        nodes: feature.properties.full_path,
        infra_type: infraType,
      });
      await autosave();
    }
  });

  async function deleteRoute() {
    if (id) {
      await $backend!.deleteRoute(id);
      await autosave();
    }
    $mode = { kind: "main" };
  }

  function onKeyDown(e: KeyboardEvent) {
    // Ignore keypresses if we're not focused on the map
    if ((e.target as HTMLElement).tagName == "INPUT") {
      return;
    }

    if (e.key == "Escape") {
      e.preventDefault();
      $mode = { kind: "main" };
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="sidebar">
    <h2>Creating/editing a route</h2>

    <button on:click={() => ($mode = { kind: "main" })} disabled={id == null}>
      Done
    </button>
    <button class="secondary" on:click={deleteRoute}>Delete</button>

    <label>
      Name:
      <input type="text" bind:value={name} />
    </label>

    <fieldset>
      <legend>Infrastructure type:</legend>
      {#each infraTypes as [value, label, _]}
        <label>
          <input type="radio" {value} bind:group={infraType} />
          {label}
        </label>
      {/each}
    </fieldset>

    <label>
      Notes:
      <textarea rows="5" bind:value={notes} />
    </label>

    {#if $routeTool}
      <hr />
      <RouteControls routeTool={$routeTool} />
    {/if}
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj}>
        <LineLayer
          id="routes"
          filter={id == null ? undefined : ["!=", ["id"], id]}
          paint={{
            "line-width": 5,
            "line-color": colorByInraType,
            "line-opacity": 0.5,
          }}
        />
      </GeoJSON>
    {/if}

    <RouteSnapperLayer />
  </div>
</SplitComponent>
