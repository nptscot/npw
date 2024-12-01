<script lang="ts">
  import { GeoJSON, LineLayer, hoverStateFilter } from "svelte-maplibre";
  import { SplitComponent } from "./common/layout";
  import {
    backend,
    mode,
    routeA,
    routeB,
    type WorstRoutes,
    type RouteGJ,
  } from "./stores";
  import Directions from "./Directions.svelte";
  import { colorByLoS } from "./colors";

  export let routes: WorstRoutes;

  let current = 0;

  let gj: RouteGJ | null = null;
  let err = "";

  async function update(current: number) {
    try {
      let route = routes[current];
      gj = await $backend!.evaluateRoute({
        start: { lng: route[0].x, lat: route[0].y },
        end: [route[1].x, route[1].y],
        breakdown: "los",
      });
      err = "";
    } catch (error: any) {
      gj = null;
      err = error.toString();
    }
  }
  $: update(current);

  function seeDetail() {
    let route = routes[current];
    $routeA = { lng: route[0].x, lat: route[0].y };
    $routeB = { lng: route[1].x, lat: route[1].y };
    $mode = { kind: "evaluate-route" };
  }
</script>

<SplitComponent>
  <div slot="left">
    <h2>Debug the worst routes</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>
    <p>
      These routes, from a sample of the OD data, have the worst directness.
    </p>

    <div>
      <button
        class="secondary"
        on:click={() => current--}
        disabled={current == 0}
      >
        Previous
      </button>
      {current + 1} / {routes.length}
      <button
        class="secondary"
        on:click={() => current++}
        disabled={current == routes.length - 1}
      >
        Next
      </button>
    </div>

    {#if err}
      <p>{err}</p>
    {/if}

    {#if gj}
      <button on:click={seeDetail}>See detail</button>

      <Directions {gj} />
    {/if}
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj} generateId>
        <LineLayer
          filter={["!", ["has", "car_route"]]}
          paint={{
            "line-width": 20,
            "line-color": colorByLoS,
            "line-opacity": hoverStateFilter(0.5, 1.0),
          }}
          manageHoverState
        />

        <LineLayer
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
</SplitComponent>
