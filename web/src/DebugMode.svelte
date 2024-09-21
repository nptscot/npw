<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { PropertiesTable, notNull } from "svelte-utils";
  import { Popup } from "svelte-utils/map";
  import { backend, mode } from "./stores";
  import { onMount } from "svelte";
  import type { FeatureCollection } from "geojson";

  let gj: FeatureCollection | null = null;
  onMount(async () => {
    gj = await $backend!.renderDebug();
  });
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Debug mode</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>
    <p>Hover to see a segment's properties, and click to open OSM</p>
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj} generateId>
        <LineLayer
          id="network"
          paint={{
            "line-width": hoverStateFilter(5, 7),
            "line-color": "black",
          }}
          manageHoverState
          on:click={(e) =>
            window.open(notNull(e.detail.features[0].properties).way, "_blank")}
          hoverCursor="pointer"
          eventsIfTopMost
        >
          <Popup openOn="hover" let:props>
            <PropertiesTable properties={props} />
          </Popup>
        </LineLayer>
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>
