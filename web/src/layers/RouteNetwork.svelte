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

  $: lineColor = {
    none: "#304ce7",
    flow: [
      "step",
      ["get", key],
      "rgba(0,0,0,0)",
      1,
      "#9C9C9C",
      50,
      "#FFFF73",
      100,
      "#AFFF00",
      250,
      "#00FFFF",
      500,
      "#30B0FF",
      1000,
      "#2E5FFF",
      2000,
      "#0000FF",
      3000,
      "#FF00C5",
    ],
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
    gradient: [
      "step",
      ["get", "gradient"],
      "#59ee19",
      3,
      "#37a009",
      5,
      "#FFC300",
      7,
      "#C70039",
      10,
      "#581845",
      100,
      "#000000",
    ],
  }[colorBy] as ExpressionSpecification;
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

    <label>
      Color by:
      <select bind:value={colorBy}>
        {#each colorByOptions as [value, label]}
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
      "line-color": lineColor,
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
