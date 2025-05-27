<script lang="ts">
  import "@scottish-government/design-system/dist/css/design-system.css";
  import "@scottish-government/design-system/dist/scripts/design-system.js";
  import type { FeatureCollection, MultiPolygon, Polygon } from "geojson";
  import { onMount } from "svelte";
  import {
    FillLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapLibre,
    NavigationControl,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import boundariesUrl from "../../assets/boundaries.geojson?url";
  import logo from "../../assets/npt_logo.png?url";
  import { stripPrefix } from "../common";
  import { listAllFiles } from "../common/files";
  import { maptilerApiKey } from "../stores";

  let gj: FeatureCollection<Polygon | MultiPolygon, { name: string }> = {
    type: "FeatureCollection" as const,
    features: [],
  };
  let lad = "";
  let ladNames: string[] = [];
  let previousBoundaries = [...listAllFiles().keys()];
  previousBoundaries.sort();

  onMount(async () => {
    // @ts-expect-error This really exists for the SG design system, but TS doesn't know about it
    window.DS.initAll();

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
    <p><a href="https://www.npt.scot/"><img src={logo} alt="NPT logo" /></a></p>

    <h1>Network Planning Workspace</h1>

    <p>
      The NPW is for designing cycle networks in Scotland.
      Building on datasets presented in the <a href="https://www.npt.scot/">Network Planning Tool (NPT)</a>, it enables planning
      infrastructure, sharing plans, and rapid insight into the benefits of changes via key metrics.
    </p>

    <div><a href="credits.html">Credits</a></div>

    <div><a href="manual.html">Manual</a></div>

    <h3><label class="ds_label" for="component">Select an area</label></h3>

    <p>Select your area from this list, or click on the map, to start.</p>

    <div class="ds_select-wrapper">
      <select
        class="ds_select"
        id="component"
        name="component"
        bind:value={lad}
      >
        <option value=""></option>
        {#each ladNames as value}
          <option {value}>{value}</option>
        {/each}
      </select>
      <span class="ds_select-arrow" aria-hidden="true"></span>
    </div>

    <button
      class="ds_button"
      on:click={() => (window.location.href = `npw.html?boundary=LAD_${lad}`)}
      disabled={lad == ""}
    >
      Start
    </button>

    {#if previousBoundaries.length > 0}
      <h3>Recently used areas</h3>

      <ul>
        {#each previousBoundaries as boundary}
          <li>
            <a href={`npw.html?boundary=${boundary}`}>
              {stripPrefix(boundary, "LAD_")}
            </a>
          </li>
        {/each}
      </ul>
    {/if}
  </div>

  <div class="map">
    <MapLibre
      style={`https://api.maptiler.com/maps/streets-v2/style.json?key=${maptilerApiKey}`}
      bounds={[-8.943, 54.631, -0.901, 59.489]}
      on:error={(e) => {
        // @ts-expect-error ErrorEvent isn't exported
        console.log(e.detail.error);
      }}
    >
      <NavigationControl showCompass={false} position="top-right" />
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

<style>
  .container {
    height: 100vh;
    position: relative;
  }

  /* Layout - existing */
  .controls {
    position: absolute;
    z-index: 1;
    left: 0;
    top: 0;
    bottom: 0;
    width: 35%;
    overflow-y: auto;
    padding: 20px;
    background-color: rgba(255, 255, 255, 0.7);
    backdrop-filter: blur(10px);
    box-shadow: 10px 0px 10px 0px rgba(0, 0, 0, 0.1);
  }
  .map {
    height: 100%;
    width: 100%;
  }
</style>
