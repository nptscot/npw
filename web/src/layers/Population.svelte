<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import { FillLayer, GeoJSON, hoverStateFilter } from "svelte-maplibre";
  import { makeRamp, Popup } from "svelte-utils/map";
  import { deprived, population } from "../colors";
  import { layerId } from "../common";
  import {
    backend,
    referenceRoadStyle,
    type ReferenceRoadStyle,
  } from "../stores";

  function fillColor(
    referenceRoadStyle: ReferenceRoadStyle,
  ): DataDrivenPropertyValueSpecification<string> {
    if (referenceRoadStyle == "population") {
      return makeRamp(
        ["get", "density_quintile"],
        population.limits,
        population.colorScale,
      );
    }
    return makeRamp(
      ["get", "imd_percentile"],
      deprived.limits,
      deprived.colorScale,
    );
  }

  function makeFilter(
    referenceRoadStyle: ReferenceRoadStyle,
  ): ExpressionSpecification {
    if (referenceRoadStyle == "population") {
      return ["<", ["get", "density_quintile"], 4];
    }
    return ["<=", ["get", "imd_percentile"], 20];
  }
</script>

{#if $backend}
  {#await $backend.getDataZones() then data}
    <GeoJSON {data} generateId>
      <FillLayer
        {...layerId("population")}
        manageHoverState
        filter={makeFilter($referenceRoadStyle)}
        paint={{
          "fill-color": fillColor($referenceRoadStyle),
          "fill-opacity": hoverStateFilter(0.7, 0.9),
        }}
        layout={{
          visibility:
            $referenceRoadStyle == "population" ||
            $referenceRoadStyle == "deprived"
              ? "visible"
              : "none",
        }}
      >
        <Popup openOn="hover" let:props>
          <p style:max-width="200px">
            Data zone {props.id}
            has {props.population.toLocaleString()}
            people, with a density of {Math.round(
              props.population / props.area_km2,
            ).toLocaleString()} people per square kilometer, putting it in quintile
            {props.density_quintile}
            for this study area. It has a SIMD rank of {props.imd_rank}, putting
            it in the {props.imd_percentile} percentile.
          </p>
        </Popup>
      </FillLayer>
    </GeoJSON>
  {/await}
{/if}
