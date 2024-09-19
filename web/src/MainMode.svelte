<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { backend, mode, routeTool } from "./stores";
  import type { FeatureCollection } from "geojson";
  import { onMount } from "svelte";

  let gj: FeatureCollection | null = null;
  onMount(async () => {
    gj = await $backend!.renderRoutes();
  });

  function newRoute() {
    $routeTool!.startRoute();
    $mode = { kind: "sketch-route", id: null };
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "r") {
      e.preventDefault();
      newRoute();
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="top">
    <button on:click={() => ($mode = { kind: "debug" })}>Debug</button>
  </div>
  <div slot="sidebar">
    <h2>Main mode</h2>
    {#if $routeTool}
      <button on:click={newRoute}>New <u>r</u>oute</button>
    {/if}
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj}>
        <LineLayer
          id="routes"
          paint={{
            "line-width": hoverStateFilter(5, 7),
            "line-color": "red",
          }}
          manageHoverState
        >
          <Popup openOn="hover" let:props>{props.name}</Popup>
        </LineLayer>
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>
