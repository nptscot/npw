<script lang="ts">
  import { layerId } from "./common";
  import {
    GeoJSON,
    hoverStateFilter,
    FillLayer,
    LineLayer,
  } from "svelte-maplibre";
  import { SplitComponent } from "./common/layout";
  import { notNull } from "svelte-utils";
  import { backend, mode } from "./stores";
</script>

<SplitComponent>
  <div slot="left">
    <h2>Debug mesh density mode</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>
  </div>

  <div slot="map">
    {#await notNull($backend).meshDensity() then data}
      <GeoJSON {data} generateId>
        <FillLayer
          {...layerId("mesh-density")}
          paint={{
            "fill-color": "grey",
            "fill-opacity": hoverStateFilter(0.5, 1.0),
          }}
          manageHoverState
        />

        <LineLayer
          {...layerId("mesh-density-outline")}
          paint={{
            "line-color": "black",
          }}
        />
      </GeoJSON>
    {/await}
  </div>
</SplitComponent>
