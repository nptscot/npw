<script lang="ts">
  import {
    CircleLayer,
    GeoJSON,
    hoverStateFilter,
    LineLayer,
  } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { PropertiesTable, notNull } from "svelte-utils";
  import { Popup } from "svelte-utils/map";
  import { backend, mode } from "./stores";
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Debug mesh density mode</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>
  </div>

  <div slot="map">
    {#await notNull($backend).meshDensity() then data}
      <GeoJSON {data} generateId>
        <LineLayer
          paint={{
            "line-width": hoverStateFilter(5, 7),
            "line-color": ["case", ["get", "forwards"], "red", "blue"],
            "line-opacity": 0.8,
            "line-offset": 5,
          }}
          manageHoverState
        >
          <Popup openOn="hover" let:props>
            <PropertiesTable properties={props} />
          </Popup>
        </LineLayer>

        <CircleLayer
          paint={{
            "circle-radius": 8,
            "circle-color": "black",
            "circle-opacity": hoverStateFilter(0.5, 1.0),
          }}
          manageHoverState
        >
          <Popup openOn="hover" let:props>
            <PropertiesTable properties={props} />
          </Popup>
        </CircleLayer>
      </GeoJSON>
    {/await}
  </div>
</SplitComponent>
