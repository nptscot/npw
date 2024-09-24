<script lang="ts">
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    FillLayer,
  } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { Popup } from "svelte-utils/map";
  import { backend, odZones, odPairs, mode } from "./stores";
  import type { Position } from "geojson";
  import centroid from "@turf/centroid";

  let zoneCenters: { [name: string]: Position } = Object.fromEntries(
    $odZones.features.map((f) => [
      f.properties!.name,
      centroid(f).geometry.coordinates,
    ]),
  );
  let flowGj = {
    type: "FeatureCollection" as const,
    features: $odPairs
      // TODO Fix in the input data
      .filter(
        ([zone1, zone2, count]) =>
          Object.hasOwn(zoneCenters, zone1) &&
          Object.hasOwn(zoneCenters, zone2),
      )
      .map(([zone1, zone2, count]) => {
        return {
          type: "Feature" as const,
          properties: { count },
          geometry: {
            type: "LineString" as const,
            coordinates: [zoneCenters[zone1], zoneCenters[zone2]],
          },
        };
      }),
  };

  // TODO tmp
  window.alert($backend!.evaluateOD($odZones, $odPairs));
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Debug OD mode</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>
  </div>

  <div slot="map">
    <GeoJSON data={$odZones} generateId>
      <FillLayer
        paint={{
          "fill-color": "black",
          "fill-opacity": hoverStateFilter(0.2, 0.8),
        }}
        manageHoverState
        eventsIfTopMost
      >
        <Popup openOn="hover" let:props>{props.name}</Popup>
      </FillLayer>

      <LineLayer
        paint={{
          "line-width": 2,
          "line-color": "black",
        }}
      />
    </GeoJSON>

    <GeoJSON data={flowGj} generateId>
      <LineLayer
        paint={{
          "line-width": 1,
          "line-color": "red",
        }}
        manageHoverState
        eventsIfTopMost
      >
        <Popup openOn="hover" let:props>{props.count}</Popup>
      </LineLayer>
    </GeoJSON>
  </div>
</SplitComponent>
