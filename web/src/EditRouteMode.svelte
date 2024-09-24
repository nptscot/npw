<script lang="ts">
  import { GeoJSON, LineLayer, MapEvents } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { backend, mode, infraTypes, autosave } from "./stores";
  import { JsRouteSnapper } from "route-snapper";
  import type { Feature, FeatureCollection, LineString } from "geojson";
  import { onMount, onDestroy } from "svelte";
  import { colorByInraType } from "./common";
  import RouteSnapperLayer from "./snapper/RouteSnapperLayer.svelte";
  import RouteControls from "./snapper/RouteControls.svelte";
  import { snapMode, undoLength, routeToolGj } from "./snapper/stores";
  import type { Map, MapMouseEvent } from "maplibre-gl";

  export let routeSnapper: JsRouteSnapper;
  export let map: Map;
  export let id: number | null;

  // TODO Move to stores
  type RouteFeature = Feature<
    LineString,
    // TODO route props too
    { waypoints: any[]; name: string; notes: string; infra_type: string }
  >;

  let name = "";
  let notes = "";
  let infraType = "Unknown";

  let existingGj: FeatureCollection | null = null;

  onMount(async () => {
    existingGj = await $backend!.renderRoutes();

    // TODO Add to MapEvents
    map.on("dragstart", onDragStart);
    map.on("mouseup", onMouseUp);

    if (id != null) {
      let feature = existingGj.features.find(
        (f) => f.id == id,
      )! as RouteFeature;
      name = feature.properties.name;
      notes = feature.properties.notes;
      infraType = feature.properties.infra_type;

      routeSnapper.editExisting(feature.properties.waypoints);
    }

    redraw();
  });

  onDestroy(async () => {
    map.off("dragstart", onDragStart);
    map.off("mouseup", onMouseUp);

    let output = routeSnapper.toFinalFeature();
    routeSnapper.clearState();

    if (!output) {
      window.alert("No route drawn");
      return;
    }
    let feature = JSON.parse(output);

    try {
      await $backend!.setRoute(id, {
        feature,
        name,
        notes,
        nodes: feature.properties.full_path,
        infra_type: infraType,
      });
    } catch (err) {
      window.alert(err);
    }
    await autosave();
  });

  async function deleteRoute() {
    if (id) {
      await $backend!.deleteRoute(id);
      await autosave();
    }
    $mode = { kind: "main" };
  }

  // TODO Maybe group this code elsewhere

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

  function onKeyPress(e: KeyboardEvent) {
    // Ignore keypresses if we're not focused on the map
    if ((e.target as HTMLElement).tagName == "INPUT") {
      return;
    }

    if (e.key == "Enter") {
      e.preventDefault();
      $mode = { kind: "main" };
    } else if (e.key == "s" || e.key == "S") {
      e.preventDefault();
      routeSnapper.toggleSnapMode();
      redraw();
    } else if (e.key == "z" && e.ctrlKey) {
      e.preventDefault();
      routeSnapper.undo();
      redraw();
    }
  }

  function redraw() {
    let gj = JSON.parse(routeSnapper.renderGeojson());
    map.getCanvas().style.cursor = gj.cursor;
    $snapMode = gj.snap_mode;
    $undoLength = gj.undo_length;
    $routeToolGj = gj;
  }

  function onMouseMove(ev: CustomEvent<MapMouseEvent>) {
    let snapDistancePixels = 30;
    let e = ev.detail;
    let nearbyPoint: [number, number] = [
      e.point.x - snapDistancePixels,
      e.point.y,
    ];
    let circleRadiusMeters = map
      .unproject(e.point)
      .distanceTo(map.unproject(nearbyPoint));
    if (
      routeSnapper.onMouseMove(e.lngLat.lng, e.lngLat.lat, circleRadiusMeters)
    ) {
      redraw();
    }
  }

  function onClick() {
    routeSnapper.onClick();
    redraw();
  }

  function onDragStart() {
    if (routeSnapper.onDragStart()) {
      map.dragPan.disable();
    }
  }

  function onMouseUp() {
    if (routeSnapper.onMouseUp()) {
      map.dragPan.enable();
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} on:keypress={onKeyPress} />

<SplitComponent>
  <div slot="sidebar">
    <h2>Editing a route</h2>

    <button on:click={() => ($mode = { kind: "main" })}>Back</button>
    <button class="secondary" on:click={deleteRoute}>Delete</button>

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

    <RouteControls {routeSnapper} />
  </div>

  <div slot="map">
    <MapEvents on:mousemove={onMouseMove} on:click={onClick} />

    {#if existingGj}
      <GeoJSON data={existingGj}>
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
