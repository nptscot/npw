<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
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

  let gj: FeatureCollection<
    Polygon | MultiPolygon,
    { kind: "LAD" | "REGION"; name: string }
  > = {
    type: "FeatureCollection" as const,
    features: [],
  };
  let ladNames: string[] = [];
  let regionNames: string[] = [];
  let kind = "LAD";

  onMount(async () => {
    let resp = await fetch(boundariesUrl);
    gj = await resp.json();

    for (let f of gj.features) {
      if (f.properties.kind == "LAD") {
        ladNames.push(f.properties.name);
      } else {
        regionNames.push(f.properties.name);
      }
    }
    ladNames.sort();
    regionNames.sort();
    ladNames = ladNames;
    regionNames = regionNames;
  });

  function onClick(e: CustomEvent<LayerClickInfo>) {
    let props = e.detail.features[0].properties!;
    window.location.href = `npw.html?boundary=${props.kind}_${props.name}`;
  }
</script>

<div class="left">
  <h2>Network Planning Workspace</h2>
  <p>
    This is an
    <a href="https://github.com/nptscot/npw" target="_blank">
      open source project
    </a>
    project developed by
    <a href="https://github.com/dabreegster/" target="_blank">Dustin Carlino</a>
    .
  </p>

  <fieldset>
    <label>
      <input type="radio" value="LAD" bind:group={kind} />
      Local Authority Districts
    </label>
    <label>
      <input type="radio" value="REGION" bind:group={kind} />
      Regions
    </label>
  </fieldset>

  <p>Choose a boundary below or on the map to begin sketching:</p>
  <ul style="columns: 3">
    {#if kind == "LAD"}
      {#each ladNames as name}
        <li><a href="npw.html?boundary=LAD_{name}">{name}</a></li>
      {/each}
    {:else}
      {#each regionNames as name}
        <li><a href="npw.html?boundary=REGION_{name}">{name}</a></li>
      {/each}
    {/if}
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

<div class="main">
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
          filter={["==", ["get", "kind"], kind]}
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
          filter={["==", ["get", "kind"], kind]}
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

<style>
  div {
    float: left;
  }

  .left {
    width: 35%;
    height: 100vh;
    overflow: scroll;
    padding: 8px;
  }

  .main {
    width: 65%;
    height: 100vh;
  }
</style>
