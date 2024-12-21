<script lang="ts">
  import { map, interactiveMapLayersEnabled } from "../stores";
  import { onDestroy } from "svelte";
  import type { MapMouseEvent } from "maplibre-gl";
  import { MapEvents } from "svelte-maplibre";

  let source: "google" | "bing" = "google";

  function start() {
    $interactiveMapLayersEnabled = false;
    if ($map) {
      $map.getCanvas().style.cursor = "crosshair";
    }
  }

  function stop() {
    $interactiveMapLayersEnabled = true;
    if ($map) {
      $map.getCanvas().style.cursor = "inherit";
    }
  }

  onDestroy(stop);

  function onClick(e: CustomEvent<MapMouseEvent>) {
    if ($interactiveMapLayersEnabled) {
      return;
    }
    let lon = e.detail.lngLat.lng;
    let lat = e.detail.lngLat.lat;
    if (source == "google") {
      window.open(
        `http://maps.google.com/maps?q=&layer=c&cbll=${lat},${lon}&cbp=11,0,0,0,0`,
        "_blank",
      );
    } else if (source == "bing") {
      window.open(
        `https://www.bing.com/maps?cp=${lat}~${lon}&style=x`,
        "_blank",
      );
    }
  }

  function onKeyDown(e: KeyboardEvent) {
    if (!$interactiveMapLayersEnabled && e.key == "Escape") {
      e.stopPropagation();
      stop();
    }
  }
</script>

<MapEvents on:click={onClick} />

<svelte:window on:keydown={onKeyDown} />

{#if $interactiveMapLayersEnabled}
  <button class="secondary" on:click={start}>StreetView</button>
{:else}
  <button class="secondary" on:click={stop}>Stop StreetView</button>

  <fieldset>
    <legend>Source:</legend>
    <label>
      <input type="radio" value="google" bind:group={source} />
      Google Street View
    </label>
    <label>
      <input type="radio" value="bing" bind:group={source} />
      Bing Streetside
    </label>
  </fieldset>
{/if}
