<script lang="ts">
  import type { MapMouseEvent } from "maplibre-gl";
  import { onDestroy } from "svelte";
  import { MapEvents } from "svelte-maplibre";
  import icon from "../../assets/streetview.svg?url";
  import { interactiveMapLayersEnabled, map } from "../stores";

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
    window.open(
      `http://maps.google.com/maps?q=&layer=c&cbll=${lat},${lon}&cbp=11,0,0,0,0`,
      "_blank",
    );
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

<div>
  {#if $interactiveMapLayersEnabled}
    <button on:click={start}>
      <img src={icon} title="StreetView" alt="StreetView" />
      StreetView
    </button>
  {:else}
    <button style:background="green" on:click={stop}>
      <img src={icon} title="StreetView" alt="StreetView" />
      Stop
    </button>
  {/if}
</div>

<style>
  div {
    position: absolute;
    bottom: 40px;
    left: 10px;
  }
</style>
