<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { backend, infraTypeMapping } from "../stores";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { QualitativeLegend } from "../common";
  import { levelOfServiceColors, colorByLoS } from "../colors";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }
</script>

<LayerControls name="level of service">
  <label>
    <input type="checkbox" bind:checked={show} />
    Level of Service
  </label>

  {#if show}
    <QualitativeLegend colors={levelOfServiceColors} />
  {/if}
</LayerControls>

{#if $backend && firstLoad}
  {#await $backend.renderLevelOfService() then data}
    <GeoJSON {data} generateId>
      <LineLayer
        layout={{
          visibility: show ? "visible" : "none",
        }}
        paint={{
          "line-width": hoverStateFilter(5, 7),
          "line-color": colorByLoS,
          "line-opacity": 0.8,
        }}
        manageHoverState
      >
        <Popup openOn="hover" let:props>
          <table>
            <tr>
              <th>Level of Service</th>
              <td>{props.los}</td>
            </tr>
            <tr>
              <th>Infrastructure type</th>
              <td>{infraTypeMapping[props.infra_type][0]}</td>
            </tr>
            <tr>
              <th>Speed (mph)</th>
              <td>{props.speed}</td>
            </tr>
            <tr>
              <th>Estimated daily traffic volume</th>
              <td>{props.traffic.toLocaleString()}</td>
            </tr>
          </table>
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
