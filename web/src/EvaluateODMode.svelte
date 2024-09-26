<script lang="ts">
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { Popup } from "svelte-utils/map";
  import { backend, odZones, odPairs, mode } from "./stores";
  import { onMount } from "svelte";
  import type { FeatureCollection } from "geojson";
  import { colorByInraType } from "./common";
  import type {
    ExpressionSpecification,
    DataDrivenPropertyValueSpecification,
  } from "maplibre-gl";

  let gj:
    | (FeatureCollection & {
        succeeded: number;
        failed: number;
        max_count: number;
      })
    | null = null;

  onMount(async () => {
    gj = await $backend!.evaluateOD($odZones, $odPairs);
  });

  function lineWidth(
    maxCount: number,
  ): DataDrivenPropertyValueSpecification<number> {
    let min = 0;

    // Linearly interpolate between thin and thick, based on the percent each count is between min and max
    let thin = 2;
    let thick = 10;

    let range_input = maxCount - min;
    let range_output = thick - thin;
    // min(1, (value - min) / range_input)
    let calculatePercent: ExpressionSpecification = [
      "min",
      1.0,
      ["/", ["-", ["get", "count"], min], range_input],
    ];
    // thin + range_output * percent
    return ["+", thin, ["*", range_output, calculatePercent]];
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Evaluate OD mode</h2>
    <button on:click={() => ($mode = { kind: "main" })}>Back</button>

    {#if gj}
      <p>
        {gj.succeeded.toLocaleString()} routes succeeded, {gj.failed.toLocaleString()}
        failed
      </p>
      <p>Highest count on any one road is {gj.max_count.toLocaleString()}</p>
    {/if}
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj} generateId>
        <LineLayer
          paint={{
            "line-width": lineWidth(gj.max_count),
            "line-color": colorByInraType,
          }}
          manageHoverState
          eventsIfTopMost
        >
          <Popup openOn="hover" let:props>
            {props.count} on {props.infra_type}
          </Popup>
        </LineLayer>
      </GeoJSON>
    {/if}
  </div>
</SplitComponent>
