<script lang="ts">
  import { CircleLayer, GeoJSON } from "svelte-maplibre";
  import { layerId } from "../common";
  import { backend } from "../stores";
  import { townCentrePoints as show } from "./stores";
</script>

{#if $backend}
  {#await $backend.getTownCentrePoints() then data}
    <GeoJSON {data}>
      <CircleLayer
        {...layerId("town-centres-points")}
        paint={{
          "circle-color": "purple",
          "circle-radius": 5,
        }}
        layout={{
          visibility: $show ? "visible" : "none",
        }}
      />
    </GeoJSON>
  {/await}
{/if}
