<script lang="ts">
  import { GeoJSON, hoverStateFilter, LineLayer } from "svelte-maplibre";
  import { backend, infraTypeMapping } from "../stores";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { constructMatchExpression } from "svelte-utils/map";

  let show = false;
  let firstLoad = false;

  $: if (show) {
    firstLoad = true;
  }
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Level of Service
  </label>
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
          "line-color": constructMatchExpression(
            ["get", "los"],
            {
              High: "mediumseagreen",
              Medium: "orange",
              Low: "red",
              ShouldNotBeUsed: "brown",
            },
            "black",
          ),
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
              <th>Average daily motor traffic</th>
              <td>{props.traffic.toLocaleString()}</td>
            </tr>
          </table>
        </Popup>
      </LineLayer>
    </GeoJSON>
  {/await}
{/if}
