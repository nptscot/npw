<script lang="ts">
  import { VectorTileSource, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { assetUrl } from "../stores";
  import type { ExpressionSpecification } from "maplibre-gl";

  let show = false;
  let purpose = "all";
  let scenario = "bicycle_go_dutch";
  let networkType = "fastest";

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

  // Implements the formula y = (3 / (1 + exp(-3*(x/1000 - 1.6))) + 0.3)
  $: lineWidth = [
    "interpolate",
    ["linear"],
    ["zoom"],
    12,
    [
      "*",
      2.1,
      [
        "+",
        0.3,
        [
          "/",
          3,
          ["+", 1, ["^", 2.718, ["-", 2.94, ["*", ["get", key], 0.0021]]]],
        ],
      ],
    ],
    14,
    [
      "*",
      5.25,
      [
        "+",
        0.3,
        [
          "/",
          3,
          ["+", 1, ["^", 2.718, ["-", 2.94, ["*", ["get", key], 0.0021]]]],
        ],
      ],
    ],
    15,
    [
      "*",
      7.5,
      [
        "+",
        0.3,
        [
          "/",
          3,
          ["+", 1, ["^", 2.718, ["-", 2.94, ["*", ["get", key], 0.0021]]]],
        ],
      ],
    ],
    16,
    [
      "*",
      18,
      [
        "+",
        0.3,
        [
          "/",
          3,
          ["+", 1, ["^", 2.718, ["-", 2.94, ["*", ["get", key], 0.0021]]]],
        ],
      ],
    ],
    18,
    [
      "*",
      52.5,
      [
        "+",
        0.3,
        [
          "/",
          3,
          ["+", 1, ["^", 2.718, ["-", 2.94, ["*", ["get", key], 0.0021]]]],
        ],
      ],
    ],
  ] as ExpressionSpecification;
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Route network
  </label>

  {#if show}
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
  {/if}
</LayerControls>

<VectorTileSource url={`pmtiles://${assetUrl("route_network.pmtiles")}`}>
  <LineLayer
    sourceLayer="rnet"
    paint={{
      "line-color": "black",
      "line-width": lineWidth,
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
