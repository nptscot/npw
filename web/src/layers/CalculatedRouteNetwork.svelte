<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { colorByInfraType, colorByLoS } from "../colors";
  import { layerId } from "../common";
  import ODBreakdowns from "../stats/ODBreakdowns.svelte";
  import { backend, mutationCounter, type EvaluateODOut } from "../stores";
  import { lineColorForDemand, lineWidthForDemand } from "../utils";
  import LayerControls from "./LayerControls.svelte";

  let show = false;
  let colorBy: "flow" | "infra_type" | "los" = "los";

  // Until we have loading screens, don't automatically update this layer
  // Start less than $mutationCounter
  let lastUpdate = 0;
  let gj: EvaluateODOut | null = null;

  async function recalc() {
    gj = await $backend!.evaluateOD();
    lastUpdate = $mutationCounter;
  }

  // First load case
  $: if (show && lastUpdate == 0) {
    recalc();
  }
</script>

<LayerControls name="Route network (calculated)" bind:show>
  <button on:click={recalc} disabled={$mutationCounter == lastUpdate}>
    Recalculate
  </button>

  <label>
    Color:
    <select bind:value={colorBy}>
      <option value="flow">Flow</option>
      <option value="los">Level of service</option>
      <option value="infra_type">Infrastructure type</option>
    </select>
  </label>

  {#if gj}
    <p>
      {gj.succeeded.toLocaleString()} routes succeeded, {gj.failed.toLocaleString()}
      failed
    </p>
    <p>Highest count on any one road is {gj.max_count.toLocaleString()}</p>

    <ODBreakdowns od={gj} />
  {/if}
</LayerControls>

{#if gj}
  <GeoJSON data={gj} generateId>
    <LineLayer
      {...layerId("calculated-rnet")}
      paint={{
        "line-width": lineWidthForDemand("count"),
        "line-color": {
          flow: lineColorForDemand("count"),
          infra_type: colorByInfraType,
          los: colorByLoS,
        }[colorBy],
      }}
      layout={{
        visibility: show ? "visible" : "none",
      }}
      manageHoverState
    >
      <Popup openOn="hover" let:props>
        {props.count.toLocaleString()} on {props.infra_type} ({props.los})
      </Popup>
    </LineLayer>
  </GeoJSON>
{/if}
