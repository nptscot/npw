<script lang="ts">
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { colorByInfraType, colorByLoS } from "./colors";
  import { layerId } from "./common";
  import { SplitComponent } from "./common/layout";
  import ODBreakdowns from "./stats/ODBreakdowns.svelte";
  import { backend, mode, type EvaluateODOut } from "./stores";
  import { lineColorForDemand, lineWidthForDemand } from "./utils";

  let gj: EvaluateODOut | null = null;

  onMount(async () => {
    gj = await $backend!.evaluateOD();
  });

  let colorBy: "flow" | "infra_type" | "los" = "flow";
</script>

<SplitComponent>
  <div slot="left">
    <h2>Evaluate OD mode</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>

    {#if gj}
      <p>
        {gj.succeeded.toLocaleString()} routes succeeded, {gj.failed.toLocaleString()}
        failed
      </p>
      <p>Highest count on any one road is {gj.max_count.toLocaleString()}</p>

      <label>
        Color:
        <select bind:value={colorBy}>
          <option value="flow">Flow</option>
          <option value="los">Level of service</option>
          <option value="infra_type">Infrastructure type</option>
        </select>
      </label>

      <ODBreakdowns od={gj} />
    {/if}
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj} generateId>
        <LineLayer
          {...layerId("eval-od-mode")}
          paint={{
            "line-width": lineWidthForDemand("count"),
            "line-color": {
              flow: lineColorForDemand("count"),
              infra_type: colorByInfraType,
              los: colorByLoS,
            }[colorBy],
          }}
          manageHoverState
        >
          <Popup openOn="hover" let:props>
            {props.count.toLocaleString()} on {props.infra_type} ({props.los})
          </Popup>
        </LineLayer>
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>
