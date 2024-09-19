<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { backend, mode } from "./stores";
  import type { FeatureCollection } from "geojson";
  import { onMount, onDestroy } from "svelte";

  export let id: number;

  let gj: FeatureCollection | null = null;
  let feature: any | null = null;

  onMount(async () => {
    gj = await $backend!.renderRoutes();
    feature = gj.features.find((f) => f.id == id)!;
  });

  onDestroy(async () => {
    if (feature) {
      // TODO This API is weird
      await $backend!.editRoute(id, {
        feature,
        name: feature.properties.name,
        notes: feature.properties.notes,
        nodes: feature.properties.full_path,
      });
    }
  });

  async function deleteRoute() {
    await $backend!.deleteRoute(id);
  }

  function onKeyDown(e: KeyboardEvent) {
    // Ignore keypresses if we're not focused on the map
    if ((e.target as HTMLElement).tagName == "INPUT") {
      return;
    }

    if (e.key == "Escape") {
      e.preventDefault();
      $mode = { kind: "main" };
    } else if (e.key == "e") {
      e.preventDefault();
      $mode = { kind: "sketch-route", id };
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="top">Nav</div>
  <div slot="sidebar">
    <h2>Route details mode</h2>

    <button on:click={() => ($mode = { kind: "main" })}>Save</button>
    <button on:click={() => ($mode = { kind: "sketch-route", id })}>
      <u>E</u>
      dit route
    </button>
    <button on:click={deleteRoute}>Delete</button>

    {#if feature}
      <label>
        Name:
        <input type="text" bind:value={feature.properties.name} />
      </label>

      <label>
        Notes:
        <textarea rows="5" bind:value={feature.properties.notes} />
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
