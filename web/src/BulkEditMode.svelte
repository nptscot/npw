<script lang="ts">
  import booleanIntersects from "@turf/boolean-intersects";
  import type {
    Feature,
    FeatureCollection,
    LineString,
    Polygon,
  } from "geojson";
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import { onDestroy, onMount } from "svelte";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { constructMatchExpression, emptyGeojson } from "svelte-utils/map";
  import { infraTypeColors, tierColors } from "./colors";
  import { QualitativeLegend } from "./common";
  import DrawRectangle from "./common/DrawRectangle.svelte";
  import { SplitComponent } from "./common/layout";
  import {
    autosave,
    backend,
    editsRoadStyle,
    map,
    mode,
    referenceRoadStyle,
  } from "./stores";
  import type { RouteProps } from "./types";

  let allRoutes = emptyGeojson() as FeatureCollection<LineString, RouteProps>;
  onMount(async () => {
    allRoutes = await $backend!.getAllRoutes();
  });

  let origEditsStyle = $editsRoadStyle;
  let origReferenceStyle = $referenceRoadStyle;
  $editsRoadStyle = "off";
  $referenceRoadStyle = "off";
  onDestroy(() => {
    $editsRoadStyle = origEditsStyle;
    $referenceRoadStyle = origReferenceStyle;
  });

  let colorBy: "infra" | "tier" =
    origEditsStyle != "edits_tier" ? "infra" : "tier";

  let selectedIds: Set<number> = new Set();
  function toggle(e: CustomEvent<LayerClickInfo>) {
    let id = e.detail.features[0].properties!.id;
    if (selectedIds.has(id)) {
      selectedIds.delete(id);
    } else {
      selectedIds.add(id);
    }
    selectedIds = selectedIds;
  }

  function newRectangle(rectangle: Feature<Polygon>) {
    for (let f of allRoutes.features) {
      if (booleanIntersects(rectangle, f)) {
        selectedIds.add(f.properties.id);
      }
    }
    selectedIds = selectedIds;
  }

  async function deleteAll() {
    if (
      window.confirm(
        `Really delete ${selectedIds.size} ${selectedIds.size == 1 ? "route" : "routes"}?`,
      )
    ) {
      await $backend!.deleteRoutes([...selectedIds]);
      await autosave();
      allRoutes = await $backend!.getAllRoutes();
      selectedIds = new Set();
    }
  }

  function lineColor(
    selectedIds: Set<number>,
    colorBy: "infra" | "tier",
  ): DataDrivenPropertyValueSpecification<string> {
    return [
      "case",
      ["in", ["get", "id"], ["literal", [...selectedIds]]],
      "blue",
      {
        infra: constructMatchExpression(
          ["get", "infra_type"],
          infraTypeColors,
          "black",
        ),
        tier: constructMatchExpression(["get", "tier"], tierColors, "black"),
      }[colorBy] as ExpressionSpecification,
    ];
  }
</script>

<SplitComponent>
  <div slot="left">
    <h2>Bulk edit</h2>

    <button on:click={() => ($mode = { kind: "main" })}>Back</button>

    <div style="border: 2px solid black; padding: 4px">
      <div><b>Your current network edits</b></div>

      <label>
        <input type="radio" value="off" bind:group={colorBy} disabled />
        Don't show
      </label>
      <label>
        <input type="radio" value="tier" bind:group={colorBy} />
        Tier
      </label>
      <label>
        <input type="radio" value="infra" bind:group={colorBy} />
        Infrastructure type
      </label>

      <details>
        <summary>Legend</summary>
        {#if colorBy == "infra"}
          <QualitativeLegend colors={infraTypeColors} />
        {:else if colorBy == "tier"}
          <QualitativeLegend colors={tierColors} horiz />
        {/if}
      </details>
    </div>

    <p>
      <kbd>Shift + click</kbd>
      and drag to select quickly
    </p>

    <p>
      You've selected {selectedIds.size}
      {selectedIds.size == 1 ? "route" : "routes"}
    </p>

    <button on:click={deleteAll} disabled={selectedIds.size == 0}>
      Delete all
    </button>
  </div>

  <div slot="map">
    <GeoJSON data={allRoutes} generateId>
      <LineLayer
        manageHoverState
        paint={{
          "line-width": hoverStateFilter(4, 6),
          "line-color": lineColor(selectedIds, colorBy),
        }}
        hoverCursor="pointer"
        on:click={toggle}
      />
    </GeoJSON>

    {#if $map}
      <DrawRectangle map={$map} {newRectangle} />
    {/if}
  </div>

  <div slot="right" />
</SplitComponent>
