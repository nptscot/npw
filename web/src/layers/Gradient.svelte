<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId, roadLineWidth } from "../common";
  import { backend, roadStyle } from "../stores";
  import { lineColorForGradient } from "../utils";
  import RoadLayerControls from "./RoadLayerControls.svelte";

  $: show = $roadStyle == "gradient";
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }

  // TODO Legend
</script>

<RoadLayerControls name="Gradient" style="gradient" />

{#if $backend && firstLoad}
  {#await $backend.renderLevelOfService() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        {...layerId("gradients")}
        layout={{
          visibility: show ? "visible" : "none",
        }}
        paint={{
          "line-width": roadLineWidth(0),
          "line-color": lineColorForGradient(),
        }}
        manageHoverState
      >
        <Popup openOn="hover" let:props>
          <p>{props.gradient.toFixed(1)}%</p>
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
