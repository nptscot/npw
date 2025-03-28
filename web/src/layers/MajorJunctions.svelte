<script lang="ts">
  import { CircleLayer, GeoJSON } from "svelte-maplibre";
  import { notNull } from "svelte-utils";
  import { layerId } from "../common";
  import { backend, mode } from "../stores";
  import { majorJunctions as show } from "./stores";

  // TODO Stay disabled until we snap to them too
  let enable = false;
</script>

{#if $backend}
  {#await notNull($backend).getMajorJunctions() then data}
    <GeoJSON {data}>
      <CircleLayer
        {...layerId("major-junctions")}
        paint={{
          "circle-color": "black",
          "circle-radius": ["interpolate", ["linear"], ["zoom"], 12, 3, 18, 5],
          "circle-opacity": 0.5,
          "circle-blur": 0.5,
        }}
        layout={{
          visibility:
            enable && $show && $mode.kind == "edit-route" ? "visible" : "none",
        }}
      />
    </GeoJSON>
  {/await}
{/if}
