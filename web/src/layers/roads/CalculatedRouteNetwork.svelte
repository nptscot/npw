<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Loading } from "svelte-utils";
  import { constructMatchExpression, Popup } from "svelte-utils/map";
  import { infraTypeColors, levelOfServiceColors } from "../../colors";
  import {
    Checkbox,
    layerId,
    lineColorForDemand,
    lineWidthForDemand,
  } from "../../common";
  import { backend, backgroundLayer, mutationCounter } from "../../stores";
  import type { EvaluateODOut } from "../../types";

  $: show = $backgroundLayer == "calculated_rnet";
  let fastSample = true;
  let colorBy: "demand" | "infra_type" | "los" = "los";
  let minDemand = 0;

  // Until we have loading screens, don't automatically update this layer
  let lastUpdate = 0;
  let lastFastSample = true;
  let gj: EvaluateODOut | null = null;
  let loading = "";

  async function recalc() {
    loading = "Evaluating OD data";
    gj = await $backend!.evaluateOD(fastSample);
    loading = "";
    lastUpdate = $mutationCounter;
    lastFastSample = fastSample;
  }

  // First load case
  $: if (show && lastUpdate == 0) {
    recalc();
  }
</script>

<Loading {loading} />

{#if show}
  <div>
    <button
      on:click={recalc}
      disabled={$mutationCounter == lastUpdate && fastSample == lastFastSample}
    >
      Recalculate
    </button>
  </div>

  <br />

  <Checkbox small bind:checked={fastSample}>
    Just sample desire lines (fast)
  </Checkbox>

  <br />

  <div>
    <label>
      Color:
      <select bind:value={colorBy}>
        <option value="demand">Demand</option>
        <option value="los">Level of service</option>
        <option value="infra_type">Infrastructure type</option>
      </select>
    </label>
  </div>

  <br />

  <div>
    <label>
      Show demand above:
      <input type="number" bind:value={minDemand} />
    </label>
  </div>

  {#if gj}
    <br />

    <p>
      {gj.succeeded.toLocaleString()} routes succeeded, {gj.failed.toLocaleString()}
      failed
    </p>
    <p>Highest count on any one road is {gj.max_count.toLocaleString()}</p>
  {/if}
{/if}

{#if gj}
  <GeoJSON data={gj} generateId>
    <LineLayer
      {...layerId("calculated-rnet")}
      filter={[">=", ["get", "count"], minDemand]}
      paint={{
        "line-width": lineWidthForDemand(["get", "count"]),
        "line-color": {
          demand: lineColorForDemand(["get", "count"]),
          infra_type: constructMatchExpression(
            ["get", "infra_type"],
            infraTypeColors,
            "black",
          ),
          los: constructMatchExpression(
            ["get", "los"],
            levelOfServiceColors,
            "black",
          ),
        }[colorBy],
      }}
      layout={{
        visibility: show ? "visible" : "none",
      }}
      manageHoverState
    >
      <Popup openOn="hover" let:props>
        {props.count.toLocaleString()} on {props.infra_type} ({props.los} level of
        service)
      </Popup>
    </LineLayer>
  </GeoJSON>
{/if}
