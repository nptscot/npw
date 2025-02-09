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
  import { Modal, QualitativeLegend } from "svelte-utils";
  import { constructMatchExpression, emptyGeojson } from "svelte-utils/map";
  import { infraTypeColors, tierColors } from "./colors";
  import DrawRectangle from "./common/DrawRectangle.svelte";
  import { SplitComponent } from "./common/layout";
  import PickInfraType from "./edit/PickInfraType.svelte";
  import {
    autosave,
    backend,
    editsRoadStyle,
    map,
    mode,
    referenceRoadStyle,
  } from "./stores";
  import { infraTypeMapping, type RouteProps } from "./types";

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

  let showTierModal = false;
  let overrideTier = "Primary";
  let overrideInfraType = "SegregatedWide";
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

    {#if selectedIds.size > 0}
      <p>They have tiers: {describeCounts(selectedTiers)}</p>
      {#if selectedTiers.length > 1}
        <button class="secondary" on:click={() => (showTierModal = true)}>
          Change tier
        </button>
      {/if}

      <p>
        They have infrastructure types: {describeCounts(selectedInfraTypes)}
      </p>
      {#if selectedInfraTypes.length > 1}
        <button class="secondary" on:click={() => (showInfraTypeModal = true)}>
          Change infrastructure type
        </button>
      {/if}
    {/if}
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

{#if showTierModal}
  <span class="pico">
    <Modal on:close={() => (showTierModal = false)}>
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

      <button on:click={changeTier}>Change tier</button>
      <button on:click={() => (showTierModal = false)}>Cancel</button>
    </Modal>
  </span>
{/if}

{#if showInfraTypeModal}
  <span class="pico">
    <Modal on:close={() => (showInfraTypeModal = false)}>
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

      <button on:click={changeInfraType}>Change infrastructure type</button>
      <button on:click={() => (showInfraTypeModal = false)}>Cancel</button>
    </Modal>
  </span>
{/if}
