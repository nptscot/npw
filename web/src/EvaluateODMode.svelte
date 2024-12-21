<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { layerId } from "./common";
  import { SplitComponent } from "./common/layout";
  import { Popup } from "svelte-utils/map";
  import { backend, mode, type EvaluateODOut } from "./stores";
  import { onMount } from "svelte";
  import { lineWidthForDemand, lineColorForDemand } from "./utils";

  let gj: EvaluateODOut | null = null;

  onMount(async () => {
    gj = await $backend!.evaluateOD();
  });
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

      <p>Percent of demand by infrastructure type:</p>
      <ul>
        <li>Off the network: {(100 * gj.percent_off_network).toFixed(1)}%</li>
        {#each Object.entries(gj.percent_on_network) as [key, percent]}
          <li>{key}: {(100 * percent).toFixed(1)}%</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj} generateId>
        <LineLayer
          {...layerId("eval-od-mode")}
          paint={{
            "line-width": lineWidthForDemand("count"),
            "line-color": lineColorForDemand("count"),
          }}
          manageHoverState
        >
          <Popup openOn="hover" let:props>
            {props.count} on {props.infra_type}
          </Popup>
        </LineLayer>
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>
