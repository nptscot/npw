<script lang="ts">
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { backend, mode, routeTool, infraTypes } from "./stores";
  import type { FeatureCollection } from "geojson";
  import { onMount } from "svelte";
  import Link from "./common/Link.svelte";
  import { colorByInraType } from "./common";

  let gj: FeatureCollection | null = null;
  onMount(async () => {
    gj = await $backend!.renderRoutes();
  });

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "r") {
      e.preventDefault();
      $mode = { kind: "sketch-route", id: null };
    }
  }

  function editRouteMap(e: CustomEvent<LayerClickInfo>) {
    $mode = { kind: "route-details", id: e.detail.features[0].id as number };
  }

  function editRouteSidebar(id: string | number | undefined) {
    $mode = { kind: "route-details", id: id as number };
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
      <button on:click={() => ($mode = { kind: "sketch-route", id: null })}>
        New <u>r</u>
        oute
      </button>
    {/if}

    {#if gj}
      <ol>
        {#each gj.features as f}
          <li>
            <Link on:click={() => editRouteSidebar(f.id)}>
              {f.properties?.name || `Untitled route ${f.id}`} ({f.properties
                ?.infra_type})
            </Link>
          </li>
        {/each}
      </ol>
    {/if}

    <hr />

    <details open>
      <summary>Legend</summary>
      <ul>
        {#each infraTypes as [_, label, color]}
          <li style:background={color}>{label}</li>
        {/each}
      </ul>
    </details>
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj}>
        <LineLayer
          id="routes"
          paint={{
            "line-width": hoverStateFilter(5, 7),
            "line-color": colorByInraType,
          }}
          manageHoverState
          hoverCursor="pointer"
          on:click={editRouteMap}
        >
          <Popup openOn="hover" let:props>
            {props.name || "Untitled"} ({props.infra_type})
          </Popup>
        </LineLayer>
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>
