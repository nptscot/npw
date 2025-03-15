<script lang="ts">
  import type { FeatureCollection, MultiPolygon, Polygon } from "geojson";
  import { onMount } from "svelte";
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import boundariesUrl from "../assets/boundaries.geojson?url";
  import { listAllFiles } from "./common/files";
  import { maptilerApiKey } from "./stores";

  let gj: FeatureCollection<Polygon | MultiPolygon, { name: string }> = {
    type: "FeatureCollection" as const,
    features: [],
  };
  let ladNames: string[] = [];

  onMount(async () => {
    let resp = await fetch(boundariesUrl);
    gj = await resp.json();

    for (let f of gj.features) {
      ladNames.push(f.properties.name);
    }
    ladNames.sort();
    ladNames = ladNames;
  });

  function onClick(e: CustomEvent<LayerClickInfo>) {
    let props = e.detail.features[0].properties!;
    window.location.href = `npw.html?boundary=LAD_${props.name}`;
  }
</script>

<div class="container">
  <div class="controls">
    <h2>Network Planning Workspace</h2>
    <p>
      This is an
      <a href="https://github.com/nptscot/npw" target="_blank">
        open source project
      </a>
      project developed by
      <a href="https://github.com/dabreegster/" target="_blank">
        Dustin Carlino
      </a>
      .
    </p>

    <p>Choose a boundary below or on the map to begin sketching:</p>
    <ul style="columns: 3">
      {#each ladNames as name}
        <li><a href="npw.html?boundary=LAD_{name}">{name}</a></li>
      {/each}
    </ul>

    <hr />

    <p>Or continue with a previously opened file:</p>

    <div style="columns: 2">
      {#each listAllFiles() as [boundary, list]}
        <div class="group">
          <h2>{boundary}</h2>
          {#each list as [filename, description]}
            <p>
              <a href={`npw.html?boundary=${boundary}&file=${filename}`}>
                {filename}
              </a>
              ({description})
            </p>
          {/each}
        </div>
      {/each}
    </div>

    <style>
      .group {
        border: 1px solid black;
        padding: 4px;
        margin-bottom: 8px;
        break-inside: avoid-column;
      }
    </style>
  </div>

  <div class="map">
    <div style="position: relative; width: 100%; height: 100%;">
      <MapLibre
        style={`https://api.maptiler.com/maps/streets-v2/style.json?key=${maptilerApiKey}`}
        standardControls
        bounds={[-8.943, 54.631, -0.901, 59.489]}
        on:error={(e) => {
          // @ts-expect-error ErrorEvent isn't exported
          console.log(e.detail.error);
        }}
      >
        <GeoJSON data={gj} generateId>
          <FillLayer
            paint={{
              "fill-color": "rgb(200, 100, 240)",
              "fill-outline-color": "rgb(200, 100, 240)",
              "fill-opacity": hoverStateFilter(0.0, 0.5),
            }}
            beforeId="Road labels"
            manageHoverState
            hoverCursor="pointer"
            on:click={onClick}
          >
            <Popup openOn="hover" let:props>
              <p>{props.name}</p>
            </Popup>
          </FillLayer>

          <LineLayer
            paint={{
              "line-color": "rgb(200, 100, 240)",
              "line-width": 2.5,
            }}
            beforeId="Road labels"
            manageHoverState
          />
        </GeoJSON>
      </MapLibre>
    </div>
  </div>
</div>

<style>
  :global(body) {
    margin: 0;
  }

  .container {
    display: flex;
    height: 100vh;
  }

  .controls {
    width: 35%;
    overflow-y: auto;
    padding: 4px;
  }

  .map {
    width: 65%;
  }
</style>
