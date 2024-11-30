<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "./common/layout";
  import { backend, mode, type WorstRoutes, type RouteGJ } from "./stores";
  import Directions from "./Directions.svelte";

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
        breakdown: "",
      });
      err = "";
    } catch (error: any) {
      gj = null;
      err = error.toString();
    }
  }
  $: update(current);
</script>

<SplitComponent>
  <div slot="left">
    <h2>Debug the worst routes</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>
    <p>
      These routes, from a sample of the OD data, have the worst directness.
    </p>

    <div>
      <button on:click={() => current--} disabled={current == 0}>
        Previous
      </button>
      {current + 1} / {routes.length}
      <button
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
      <Directions {gj} />
    {/if}
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj} generateId>
        <LineLayer
          id="eval-route"
          paint={{
            "line-width": 10,
            "line-color": "red",
          }}
          manageHoverState
        />
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>
