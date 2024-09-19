<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { backend, mode } from "./stores";
  import type { FeatureCollection } from "geojson";
  import { onMount, onDestroy } from "svelte";

  export let id: number;

  let gj: FeatureCollection | null = null;
  let props: { name: string; notes: string } | null = null;

  onMount(async () => {
    gj = await $backend!.renderRoutes();
    props = gj.features.find((f) => f.id == id)!.properties as {
      name: string;
      notes: string;
    };
  });

  onDestroy(async () => {
    if (props) {
      await $backend!.editRouteDetails(id, props.name, props.notes);
    }
  });

  async function deleteRoute() {
    await $backend!.deleteRoute(id);
  }
</script>

<SplitComponent>
  <div slot="top">Nav</div>
  <div slot="sidebar">
    <h2>Route details mode</h2>

    <button on:click={() => ($mode = { kind: "main" })}>Save</button>
    <button on:click={deleteRoute}>Delete</button>

    {#if props}
      <label>
        Name:
        <input type="text" bind:value={props.name} />
      </label>

      <label>
        Notes:
        <textarea rows="5" bind:value={props.notes} />
      </label>
    {/if}
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj}>
        <LineLayer
          id="routes"
          paint={{
            "line-width": 5,
            "line-color": "red",
            "line-opacity": ["case", ["==", ["id"], id], 1.0, 0.5],
          }}
        />
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>
