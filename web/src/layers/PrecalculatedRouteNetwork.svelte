<script lang="ts">
  import type { ExpressionSpecification } from "maplibre-gl";
  import { LineLayer, VectorTileSource } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { layerId } from "../common";
  import { assetUrl } from "../stores";
  import {
    lineColorForDemand,
    lineColorForGradient,
    lineWidthForDemand,
  } from "../utils";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let purpose = "all";
  let scenario = "bicycle_go_dutch";
  let networkType = "fastest";
  let colorBy = "flow";

  $: key = `${purpose}_${networkType}_${scenario}`;

  let purposes = [
    ["all", "All"],
    ["commute", "Commute"],
    ["primary", "Primary School"],
    ["secondary", "Secondary"],
    ["utility", "Other everyday"],
  ];
  let scenarios = [
    ["bicycle", "Baseline"],
    ["bicycle_go_dutch", "Go Dutch"],
    ["bicycle_ebike", "E-bike"],
  ];
  let networkTypes = [
    ["fastest", "Fast/Direct"],
    ["quietest", "Quiet/Indirect"],
  ];
  let colorByOptions = [
    ["none", "None"],
    ["flow", "People cycling per day"],
    ["quietness", "Cycle friendliness"],
    ["gradient", "Gradient"],
  ];

  $: lineColor = {
    none: "#304ce7",
    flow: lineColorForDemand(key),
    quietness: [
      "step",
      ["get", "quietness"],
      "#882255",
      25,
      "#CC6677",
      50,
      "#44AA99",
      75,
      "#117733",
      101,
      "#000000",
    ],
    gradient: lineColorForGradient,
  }[colorBy] as ExpressionSpecification;
</script>

<LayerControls name="Route network (precalculated)" bind:show>
  <label>
    Trip purpose:
    <select bind:value={purpose}>
      {#each purposes as [value, label]}
        <option {value}>{label}</option>
      {/each}
    </select>
  </label>

  <label>
    Scenario:
    <select bind:value={scenario}>
      {#each scenarios as [value, label]}
        <option {value}>{label}</option>
      {/each}
    </select>
  </label>

  <label>
    Network type:
    <select bind:value={networkType}>
      {#each networkTypes as [value, label]}
        <option {value}>{label}</option>
      {/each}
    </select>
  </label>

  <label>
    Color by:
    <select bind:value={colorBy}>
      {#each colorByOptions as [value, label]}
        <option {value}>{label}</option>
      {/each}
    </select>
  </label>
</LayerControls>

<VectorTileSource url={`pmtiles://${assetUrl("route_network.pmtiles")}`}>
  <LineLayer
    {...layerId("precalculated-rnet")}
    sourceLayer="rnet"
    paint={{
      "line-color": lineColor,
      "line-width": lineWidthForDemand(key),
    }}
    layout={{
      visibility: show ? "visible" : "none",
    }}
    hoverCursor="pointer"
  >
    <Popup openOn="click" let:props>
      <div style="max-width: 30vw; max-height: 60vh; overflow: auto;">
        <p>Cyclists: {props[key].toLocaleString()}</p>
        <p>Gradient: {props.gradient}%</p>
        <p>Cycle-friendliness: {props.quietness}%</p>

        <details>
          <summary>All network details</summary>

          <p>Fast/Direct network</p>
          <table>
            <tr>
              <td />
              <th>Baseline</th>
              <th>Go Dutch</th>
              <th>E-bikes</th>
            </tr>
            {#each purposes as [value, label]}
              <tr>
                <th>{label}</th>
                <td>{props[`${value}_fastest_bicycle`].toLocaleString()}</td>
                <td>
                  {props[`${value}_fastest_bicycle_go_dutch`].toLocaleString()}
                </td>
                <td>
                  {props[`${value}_fastest_bicycle_ebike`].toLocaleString()}
                </td>
              </tr>
            {/each}
          </table>

          <p>Quiet/Indirect network</p>
          <table>
            <tr>
              <td />
              <th>Baseline</th>
              <th>Go Dutch</th>
              <th>E-bikes</th>
            </tr>
            {#each purposes as [value, label]}
              <tr>
                <th>{label}</th>
                <td>{props[`${value}_quietest_bicycle`].toLocaleString()}</td>
                <td>
                  {props[`${value}_quietest_bicycle_go_dutch`].toLocaleString()}
                </td>
                <td>
                  {props[`${value}_quietest_bicycle_ebike`].toLocaleString()}
                </td>
              </tr>
            {/each}
          </table>
        </details>
      </div>
    </Popup>
  </LineLayer>
</VectorTileSource>
