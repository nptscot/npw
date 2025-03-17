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
  let lad = "";
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
      Welcome to the Network Planning Workspace, incorporating the Network
      Planning Tool.
    </p>

    <p>
      The NPW is designed to enable local authorities to plan a cycle network
      for the area, using segregated infrastructure on key routes and ensuring
      local places are properly connected without severance.
    </p>

    <h3>Select an area</h3>

    <p>Select your area from this list, or click on the map, to start.</p>

    <select bind:value={lad}>
      <option value=""></option>
      {#each ladNames as value}
        <option {value}>{value}</option>
      {/each}
    </select>

    <button
      on:click={() => (window.location.href = `npw.html?boundary=LAD_${lad}`)}
      disabled={lad == ""}
    >
      Start
    </button>

    <hr />

    <h3>Or continue with a previously opened file</h3>

    <div style="columns: 2">
      {#each listAllFiles() as [boundary, list]}
        <div class="group">
          <h4>{boundary}</h4>
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

  .group {
    border: 1px solid black;
    padding: 4px;
    margin-bottom: 8px;
    break-inside: avoid-column;
  }
</style>
