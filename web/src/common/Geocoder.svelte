<script lang="ts">
  // TODO Upstream back to svelte-utils
  import { createMapLibreGlMapController } from "@maptiler/geocoding-control/maplibregl";
  import GeocodingControl from "@maptiler/geocoding-control/svelte/GeocodingControl.svelte";
  import type { MapController } from "@maptiler/geocoding-control/types";
  import maplibregl, { type Map } from "maplibre-gl";

  export let apiKey: string;
  export let map: Map | null;
  export let country: string | undefined;

  // From https://docs.maptiler.com/cloud/api/geocoding/#PlaceType. poi is excluded by default.
  let allTypes = [
    "continental_marine",
    "country",
    "major_landform",
    "region",
    "subregion",
    "county",
    "joint_municipality",
    "joint_submunicipality",
    "municipality",
    "municipal_district",
    "locality",
    "neighbourhood",
    "place",
    "postal_code",
    "address",
    "road",
    "poi",
  ];

  $: mapController = setup(map);

  function setup(map: Map | null): MapController | null {
    if (!map) {
      return null;
    }
    let marker = true;
    let showResultMarkers = true;
    let flyToOptions = {
      duration: 1000,
    };
    let fitBoundsOptions = {
      duration: 1000,
    };
    return createMapLibreGlMapController(
      map,
      maplibregl,
      marker,
      showResultMarkers,
      flyToOptions,
      fitBoundsOptions,
    );
  }
</script>

{#if mapController}
  <div>
    <GeocodingControl
      {mapController}
      {apiKey}
      {country}
      types={allTypes}
      proximity={[
        {
          type: "map-center",
        },
      ]}
    />
  </div>
{/if}

<style>
  div {
    position: absolute;
    top: 10px;
    left: 50px;
  }
</style>
