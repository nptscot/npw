<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import {
    MapEvents,
    GeoJSON,
    LineLayer,
    Marker,
    hoverStateFilter,
  } from "svelte-maplibre";
  import { SplitComponent } from "./common/layout";
  import { notNull } from "svelte-utils";
  import {
    mode,
    backend,
    routeA,
    routeB,
    type Mode,
    type RouteGJ,
    type WorstRoutes,
  } from "./stores";
  import {
    colorByInfraType,
    colorByLoS,
    levelOfServiceColors,
    gradientColors,
    colorByGradientGroup,
  } from "./colors";
  import { layerId, QualitativeLegend } from "./common";
  import Directions from "./Directions.svelte";
  import { currentNetwork } from "./layers/stores";

  export let prevMode: Mode;
  export let browse: WorstRoutes;
  let currentBrowse = 0;

  let gj: RouteGJ | null = null;
  let err = "";
  let breakdown: "" | "los" | "infra_type" | "gradient" = "los";

  async function update(
    start: { lng: number; lat: number },
    end: { lng: number; lat: number },
    breakdown: "" | "los" | "infra_type" | "gradient" = "",
  ) {
    try {
      gj = await $backend!.evaluateRoute({
        start,
        end: [end.lng, end.lat],
        breakdown,
      });
      err = "";
    } catch (error: any) {
      gj = null;
      err = error.toString();
    }
  }
  $: update($routeA!, $routeB!, breakdown);

  function onRightClick(e: CustomEvent<MapMouseEvent>) {
    // Move the first marker, for convenience
    $routeA = e.detail.lngLat;
  }

  async function updateBrowse(currentBrowse: number) {
    if (browse.length == 0) {
      return;
    }
    let route = browse[currentBrowse];
    $routeA = { lng: route[0].x, lat: route[0].y };
    $routeB = { lng: route[1].x, lat: route[1].y };
    // Reactivity not working for some reason
    update($routeA, $routeB, breakdown);
  }
  $: updateBrowse(currentBrowse);

  function keyDown(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.stopPropagation();
      $mode = { kind: "main" };
    }
  }
</script>

<svelte:window on:keydown={keyDown} />

<SplitComponent>
  <div slot="left">
    <h2>Evaluate a route mode</h2>

    <button on:click={() => ($mode = prevMode)}>Back</button>

    <p>
      Move the <b>A</b>
      and
      <b>B</b>
      pins to find a route. (Hint: right-click to set the first pin somewhere.)
    </p>

    {#if err}
      <p>{err}</p>
    {/if}

    <label>
      Show details along route
      <select bind:value={breakdown}>
        <option value="">Just show route</option>
        <option value="los">Level of service</option>
        <option value="infra_type">Infrastructure type</option>
        <option value="gradient">Gradient</option>
      </select>
    </label>

    {#if breakdown == "los"}
      <QualitativeLegend colors={levelOfServiceColors} />
    {:else if breakdown == "gradient"}
      <QualitativeLegend colors={gradientColors} />
    {/if}

    {#if gj}
      <Directions {gj} />
    {/if}
  </div>

  <div slot="map">
    <MapEvents on:contextmenu={onRightClick} />

    {#await notNull($backend).renderRoutes() then data}
      <GeoJSON {data}>
        <LineLayer
          {...layerId("eval-current-routes")}
          paint={{
            "line-width": 5,
            "line-color": colorByInfraType,
          }}
          layout={{
            visibility: $currentNetwork ? "visible" : "none",
          }}
        />
      </GeoJSON>
    {/await}

    {#if $routeA && $routeB}
      <Marker bind:lngLat={$routeA} draggable>
        <span class="dot">A</span>
      </Marker>
      <Marker bind:lngLat={$routeB} draggable>
        <span class="dot">B</span>
      </Marker>
    {/if}

    {#if gj}
      <GeoJSON data={gj} generateId>
        <LineLayer
          {...layerId("eval-route-breakdown")}
          filter={["!", ["has", "car_route"]]}
          paint={{
            "line-width": 20,
            "line-color": {
              "": "cyan",
              los: colorByLoS,
              infra_type: colorByInfraType,
              gradient: colorByGradientGroup,
            }[breakdown],
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
          manageHoverState
        />

        <LineLayer
          {...layerId("eval-car-route")}
          filter={["has", "car_route"]}
          paint={{
            "line-width": 10,
            "line-color": "red",
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
          manageHoverState
        />
      </GeoJSON>
    {/if}
  </div>

  <div slot="right">
    {#if browse.length > 0}
      <p>
        These routes, from a sample of the OD data, have the worst directness.
      </p>

      <div>
        <button
          class="secondary"
          on:click={() => currentBrowse--}
          disabled={currentBrowse == 0}
        >
          Previous
        </button>
        {currentBrowse + 1} / {browse.length}
        <button
          class="secondary"
          on:click={() => currentBrowse++}
          disabled={currentBrowse == browse.length - 1}
        >
          Next
        </button>
      </div>
    {/if}
  </div>
</SplitComponent>

<style>
  .dot {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    display: flex;
    justify-content: center;
    align-items: center;

    color: white;
    background-color: blue;
    font-weight: bold;
  }
</style>
