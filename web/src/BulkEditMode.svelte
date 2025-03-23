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
  import { DrawRectangle, Modal } from "./common";
  import { SplitComponent } from "./common/layout";
  import PickInfraType from "./edit/PickInfraType.svelte";
  import { showNetworkInfraTypes, showNetworkTiers } from "./layers/stores";
  import {
    autosave,
    backend,
    editsRoadStyle,
    map,
    mode,
    referenceRoadStyle,
    type EditsRoadStyle,
  } from "./stores";
  import { infraTypeMapping, type RouteProps } from "./types";

  let allRoutes = emptyGeojson() as FeatureCollection<LineString, RouteProps>;
  onMount(async () => {
    allRoutes = await $backend!.getAllRoutes();
  });

  let origReferenceStyle = $referenceRoadStyle;
  $referenceRoadStyle = "off";
  onDestroy(() => {
    $referenceRoadStyle = origReferenceStyle;
  });

  let showTierModal = false;
  let overrideTier = "Primary";
  let overrideInfraType = "Segregated";
  let showInfraTypeModal = false;

  let selectedIds: Set<number> = new Set();

  $: selectedRoutes = allRoutes.features.filter((f) =>
    selectedIds.has(f.properties.id),
  );
  $: selectedTiers = countCases(selectedRoutes.map((f) => f.properties.tier));
  $: selectedInfraTypes = countCases(
    selectedRoutes.map((f) => infraTypeMapping[f.properties.infra_type][0]),
  );

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
      // Apply filters
      if (
        $editsRoadStyle == "edits_infra" &&
        !$showNetworkInfraTypes[f.properties.infra_type]
      ) {
        continue;
      }
      if (
        $editsRoadStyle == "edits_tier" &&
        !$showNetworkTiers[f.properties.tier]
      ) {
        continue;
      }

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
    colorBy: EditsRoadStyle,
  ): DataDrivenPropertyValueSpecification<string> {
    return [
      "case",
      ["in", ["get", "id"], ["literal", [...selectedIds]]],
      "blue",
      {
        // Unselected routes are filtered out elsewhere
        off: "black",
        edits_infra: constructMatchExpression(
          ["get", "infra_type"],
          infraTypeColors,
          "black",
        ),
        edits_tier: constructMatchExpression(
          ["get", "tier"],
          tierColors,
          "black",
        ),
      }[colorBy] as ExpressionSpecification,
    ];
  }

  function countCases(values: string[]): [string, number][] {
    let counts = new Map();
    for (let x of values) {
      if (counts.has(x)) {
        counts.set(x, counts.get(x) + 1);
      } else {
        counts.set(x, 1);
      }
    }
    let list: [string, number][] = [...counts];
    list.sort((a, b) => b[1] - a[1]);
    return list;
  }

  function describeCounts(list: [string, number][]): string {
    return list.map(([x, count]) => `${x} (${count})`).join(", ");
  }

  async function changeTier() {
    await $backend!.changeTier([...selectedIds], overrideTier);
    window.alert("Tier changed");
    await autosave();
    allRoutes = await $backend!.getAllRoutes();
    showTierModal = false;
  }

  async function changeInfraType() {
    await $backend!.changeInfraType([...selectedIds], overrideInfraType);
    window.alert("Infrastructure type changed");
    await autosave();
    allRoutes = await $backend!.getAllRoutes();
    showInfraTypeModal = false;
  }

  function makeFilter(
    _a: any,
    _b: any,
    _c: any,
    _d: any,
  ): ExpressionSpecification {
    let filters = ["any", ["in", ["get", "id"], ["literal", [...selectedIds]]]];
    if ($editsRoadStyle == "edits_infra") {
      let include = Object.keys($showNetworkInfraTypes).filter(
        // @ts-expect-error ignore
        (k) => $showNetworkInfraTypes[k],
      );
      filters.push([
        "in",
        ["get", "infra_type"],
        // @ts-expect-error ignore
        ["literal", include],
      ]);
    } else if ($editsRoadStyle == "edits_tier") {
      let include = Object.keys($showNetworkTiers).filter(
        // @ts-expect-error ignore
        (k) => $showNetworkTiers[k],
      );
      filters.push([
        "in",
        ["get", "tier"],
        // @ts-expect-error ignore
        ["literal", include],
      ]);
    }
    // @ts-expect-error ignore
    return filters;
  }
</script>

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Bulk edit</h2>
      </header>

      <div>
        <button class="ds_link" on:click={() => ($mode = { kind: "main" })}>
          &lt; Back
        </button>
      </div>

      <p>
        <b>Shift + click</b>
        and drag to select quickly
      </p>

      <p>
        You've selected {selectedIds.size}
        {selectedIds.size == 1 ? "route" : "routes"}
      </p>

      <div class="ds_button-group">
        <button
          class="ds_button ds_button--secondary"
          on:click={() => (selectedIds = new Set())}
          disabled={selectedIds.size == 0}
        >
          Clear selection
        </button>
        <button
          class="ds_button ds_button--secondary"
          on:click={deleteAll}
          disabled={selectedIds.size == 0}
        >
          Delete all
        </button>
      </div>

      {#if selectedIds.size > 0}
        <p>They have tiers: {describeCounts(selectedTiers)}</p>
        {#if selectedTiers.length > 1}
          <button
            class="ds_button ds_button--secondary"
            on:click={() => (showTierModal = true)}
          >
            Change tier
          </button>
        {/if}

        <p>
          They have infrastructure types: {describeCounts(selectedInfraTypes)}
        </p>
        {#if selectedInfraTypes.length > 1}
          <button
            class="ds_button ds_button--secondary"
            on:click={() => (showInfraTypeModal = true)}
          >
            Change infrastructure type
          </button>
        {/if}
      {/if}
    </div>

    <Modal bind:show={showTierModal}>
      <p>
        The routes you've selected have tiers: {describeCounts(selectedTiers)}
      </p>

      <label>
        Change their tier to:
        <select bind:value={overrideTier}>
          <option value="Primary">Primary routes</option>
          <option value="Secondary">Secondary routes</option>
          <option value="LocalAccess">Local access routes</option>
          <option value="LongDistance">Long distance routes</option>
        </select>
      </label>

      <div class="ds_button-group">
        <button class="ds_button" on:click={changeTier}>Change tier</button>
        <button
          class="ds_button ds_button--secondary"
          on:click={() => (showTierModal = false)}
        >
          Cancel
        </button>
      </div>
    </Modal>

    <Modal bind:show={showInfraTypeModal}>
      <p>
        The routes you've selected have infrastructure types: {describeCounts(
          selectedInfraTypes,
        )}
      </p>
      <p>
        You can override the infrastructure type for these routes, instead of
        automatically picking the most appropriate type. If you do this, you're
        making the promise that this type is appropriate to achieve high Level
        of Service.
      </p>

      <PickInfraType bind:current={overrideInfraType} />

      <div class="ds_button-group">
        <button class="ds_button" on:click={changeInfraType}>
          Change infrastructure type
        </button>
        <button
          class="ds_button ds_button--secondary"
          on:click={() => (showInfraTypeModal = false)}
        >
          Cancel
        </button>
      </div>
    </Modal>
  </div>

  <div slot="map">
    <GeoJSON data={allRoutes} generateId>
      <LineLayer
        manageHoverState
        paint={{
          "line-width": hoverStateFilter(4, 6),
          "line-color": lineColor(selectedIds, $editsRoadStyle),
        }}
        filter={makeFilter(
          selectedIds,
          $editsRoadStyle,
          $showNetworkInfraTypes,
          $showNetworkTiers,
        )}
        hoverCursor="pointer"
        on:click={toggle}
      />
    </GeoJSON>

    {#if $map}
      <DrawRectangle map={$map} {newRectangle} />
    {/if}
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
