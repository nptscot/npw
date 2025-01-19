<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { notNull, PropertiesTable } from "svelte-utils";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "./common";
  import { SplitComponent } from "./common/layout";
  import { backend, mode } from "./stores";
</script>

<SplitComponent>
  <div slot="left">
    <h2>Debug network mode</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>
    <p>Hover to see a segment's properties, and click to open OSM</p>
  </div>

  <div slot="map">
    {#await notNull($backend).renderStaticRoads() then data}
      <GeoJSON {data} generateId>
        <LineLayer
          {...layerId("debug-mode")}
          paint={{
            "line-width": hoverStateFilter(5, 7),
            "line-color": "black",
          }}
          manageHoverState
          on:click={(e) =>
            window.open(notNull(e.detail.features[0].properties).way, "_blank")}
          hoverCursor="pointer"
        >
          <Popup openOn="hover" let:props>
            <PropertiesTable properties={props} />
          </Popup>
        </LineLayer>
      </GeoJSON>
    {/await}
  </div>
</SplitComponent>
