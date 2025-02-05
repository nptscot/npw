<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapEvents,
    Marker,
  } from "svelte-maplibre";
  import { constructMatchExpression } from "svelte-utils/map";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
  } from "./colors";
  import { layerId, QualitativeLegend } from "./common";
  import { SplitComponent } from "./common/layout";
  import Directions from "./Directions.svelte";
  import AllControls from "./layers/AllControls.svelte";
  import PickEditsStyle from "./layers/roads/PickEditsStyle.svelte";
  import { backend, mode, routeA, routeB, type Mode } from "./stores";
  import type { RouteGJ, WorstRoutes } from "./types";

  export let prevMode: Mode;
  export let browse: WorstRoutes;
  let currentBrowse = 0;

  let gj: RouteGJ | null = null;
  let err = "";
  let breakdown: "" | "los" | "infra_type" | "gradient" = "los";
  let showQuietBikeRoute = false;
  let showCarRoute = false;

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
    <h2>Evaluate a route</h2>

    <button on:click={() => ($mode = prevMode)}>Back</button>

    <p>
      Move the <b>A</b>
      and
      <b>B</b>
      pins to find a route. (Hint: right-click to set the first pin somewhere.)
    </p>
    <p>
      Note the direct route is shown, ignoring bad infrastructure. This is to
      emphasize whether or not that direct route has been adequately improved by
      your network edits so far.
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
      <QualitativeLegend horiz colors={levelOfServiceColors} />
    {:else if breakdown == "gradient"}
      <QualitativeLegend colors={gradientColors} />
    {/if}

    {#if gj}
      <label>
        <input type="checkbox" bind:checked={showCarRoute} />
        <b>{(gj.direct_bike_length / gj.car_length).toFixed(1)}x</b>
        longer than the driving route (in
        <span style:color="red">red</span>
        )
      </label>

      <label>
        <input type="checkbox" bind:checked={showQuietBikeRoute} />
        <b>{(gj.direct_bike_length / gj.quiet_bike_length).toFixed(1)}x</b>
        longer than the quiet cycling route (in
        <span style:color="blue">blue</span>
        )
      </label>

      <hr />

      <Directions {gj} />
    {/if}
  </div>

  <div slot="map">
    <MapEvents on:contextmenu={onRightClick} />

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
          filter={["==", ["get", "kind"], "bicycle_direct"]}
          paint={{
            "line-width": 20,
            "line-color": {
              "": "cyan",
              los: constructMatchExpression(
                ["get", "los"],
                levelOfServiceColors,
                "black",
              ),
              infra_type: constructMatchExpression(
                ["get", "infra_type"],
                infraTypeColors,
                "black",
              ),
              gradient: constructMatchExpression(
                ["get", "gradient_group"],
                gradientColors,
                "black",
              ),
            }[breakdown],
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
          manageHoverState
        />

        <LineLayer
          {...layerId("eval-car-route")}
          filter={["==", ["get", "kind"], "car"]}
          layout={{
            visibility: showCarRoute ? "visible" : "none",
          }}
          paint={{
            "line-width": 10,
            "line-color": "red",
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
          manageHoverState
        />

        <LineLayer
          {...layerId("eval-quiet-bike-route")}
          filter={["==", ["get", "kind"], "quiet_bike"]}
          layout={{
            visibility: showQuietBikeRoute ? "visible" : "none",
          }}
          paint={{
            "line-width": 10,
            "line-color": "blue",
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

    <PickEditsStyle />
    <AllControls />
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
