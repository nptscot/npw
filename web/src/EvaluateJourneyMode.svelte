<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    MapEvents,
    Marker,
  } from "svelte-maplibre";
  import { QualitativeLegend } from "svelte-utils";
  import { constructMatchExpression } from "svelte-utils/map";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
  } from "./colors";
  import { layerId, prettyPrintDistance } from "./common";
  import { SplitComponent } from "./common/layout";
  import RelevantLayers from "./layers/RelevantLayers.svelte";
  import { backend, mode, routeA, routeB } from "./stores";
  import type { RouteGJ, Step, WorstRoutes } from "./types";

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

  let byInfraType = (step: Step) => step.infra_type;
  let byLos = (step: Step) => step.los;

  function numChanges(gj: RouteGJ, key: (step: Step) => string): number {
    let count = 0;
    // No windows(2)?
    for (let i = 0; i < gj.directions.length - 1; i++) {
      let x1 = key(gj.directions[i]);
      let x2 = key(gj.directions[i + 1]);
      if (x1 != x2) {
        count++;
      }
    }
    return count;
  }

  function percentages(gj: RouteGJ, key: (step: Step) => string): string {
    let lengthByKey: { [name: string]: number } = {};
    for (let step of gj.directions) {
      let x = key(step);
      if (!Object.hasOwn(lengthByKey, x)) {
        lengthByKey[x] = 0;
      }
      lengthByKey[x] += step.length;
    }
    let total = 0;
    for (let length of Object.values(lengthByKey)) {
      total += length;
    }

    let results = [];
    for (let [x, length] of Object.entries(lengthByKey).toSorted(
      (a, b) => b[1] - a[1],
    )) {
      let percent = Math.round((length / total) * 100);
      results.push(`${percent}% ${x}`);
    }
    return results.join(", ");
  }
</script>

<svelte:window on:keydown={keyDown} />

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Evaluate a journey</h2>
      </header>

      <div>
        <button class="ds_link" on:click={() => ($mode = { kind: "main" })}>
          &lt; Back
        </button>
      </div>

      {#if browse.length > 0}
        <p>
          These routes, from a sample of the OD data, have the worst directness.
        </p>

        <div>
          <button
            on:click={() => currentBrowse--}
            disabled={currentBrowse == 0}
          >
            Previous
          </button>
          {currentBrowse + 1} / {browse.length}
          <button
            on:click={() => currentBrowse++}
            disabled={currentBrowse == browse.length - 1}
          >
            Next
          </button>
        </div>
      {/if}

      <p>
        Move the <b>A</b>
        and
        <b>B</b>
        pins. (Hint: right-click to set the first pin somewhere.)
      </p>
      <p>
        Note the direct route is shown, ignoring bad infrastructure. This is to
        emphasize whether or not that direct route has been adequately improved
        by your network edits so far.
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
        <QualitativeLegend horiz colors={gradientColors} />
      {/if}

      {#if gj}
        <h3>Directness</h3>

        <p>
          Direct cycling route: {prettyPrintDistance(gj.direct_bike_length)}
        </p>

        <p>
          Straight as-the-crow-flies line: {prettyPrintDistance(
            gj.straight_line_length,
          )}
        </p>

        <div>
          <label>
            <input type="checkbox" bind:checked={showCarRoute} />
            <span style:color="purple">Driving route</span>
            : {prettyPrintDistance(gj.car_length)} is
            <b>{(gj.car_length / gj.direct_bike_length).toFixed(1)}x</b>
            longer than the direct cycling route
          </label>
        </div>

        <div>
          <label>
            <input type="checkbox" bind:checked={showQuietBikeRoute} />
            <span style:color="blue">Quiet cycling route</span>
            : {prettyPrintDistance(gj.quiet_bike_length)} is
            <b>{(gj.quiet_bike_length / gj.direct_bike_length).toFixed(1)}x</b>
            longer than the direct cycling route
          </label>
        </div>

        <h3>Stats</h3>

        <p>{numChanges(gj, byInfraType)} changes in infrastructure type</p>
        <p>By length: {percentages(gj, byInfraType)}</p>

        <hr />

        <p>{numChanges(gj, byLos)} changes in level of service</p>
        <p>By length: {percentages(gj, byLos)}</p>
      {/if}

      <hr />

      <RelevantLayers />
    </div>
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
            "line-color": "purple",
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

  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
