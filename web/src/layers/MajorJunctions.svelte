<script lang="ts">
  import { CircleLayer, GeoJSON } from "svelte-maplibre";
  import { notNull } from "svelte-utils";
  import { layerId } from "../common";
  import { backend, mode } from "../stores";
  import { majorJunctions as show } from "./stores";
</script>

{#if $backend}
  {#await notNull($backend).getMajorJunctions() then data}
    <GeoJSON {data}>
      <CircleLayer
        {...layerId("major-junctions")}
        paint={{
          "circle-color": "black",
          "circle-radius": ["step", ["zoom"], 0, 12, 1, 14, 3, 15, 5],
          "circle-opacity": 0.5,
          "circle-blur": 0.5,
        }}
        layout={{
          visibility: $show && $mode.kind == "edit-route" ? "visible" : "none",
        }}
      />
    </GeoJSON>
  {/await}
{/if}
