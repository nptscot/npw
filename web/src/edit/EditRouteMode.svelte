<script lang="ts">
  import type { DataDrivenPropertyValueSpecification, Map } from "maplibre-gl";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { constructMatchExpression, emptyGeojson } from "svelte-utils/map";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
    tierColors,
    tierLabels,
  } from "../colors";
  import { layerId, Modal, percent } from "../common";
  import RelevantLayers from "../layers/RelevantLayers.svelte";
  import { majorJunctions } from "../layers/stores";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import {
    autosave,
    backend,
    currentStage,
    editModeBreakdown,
    mode,
  } from "../stores";
  import type { AutosplitRoute, InfraType, Waypoint } from "../types";
  import PickInfraType from "./PickInfraType.svelte";
  import RouteControls from "./RouteControls.svelte";
  import SectionDiagram from "./SectionDiagram.svelte";
  import { waypoints } from "./stores";

  export let map: Map;
  export let id: number | null;

  let name = "";
  let notes = "";
  // This is not meaningful when overrideInfraType is false
  let infraType = "MixedTraffic";
  let overrideInfraType = false;
  let tier = $currentStage == "assessment" ? "Primary" : $currentStage;

  let showOverrideModal = false;

  let sectionsGj: AutosplitRoute = emptyGeojson() as AutosplitRoute;
  $: recalculateSections($waypoints, overrideInfraType, infraType);

  onMount(async () => {
    $waypoints = [];
    if (id != null) {
      let feature = await $backend!.getRoute(id);
      name = feature.properties.name;
      notes = feature.properties.notes;
      infraType = feature.properties.infra_type;
      overrideInfraType = feature.properties.override_infra_type;
      tier = feature.properties.tier;

      $waypoints = feature.properties.waypoints;
    }
  });

  async function deleteRoute() {
    if (id != null) {
      await $backend!.deleteRoutes([id]);
      await autosave();
    }
    $mode = { kind: "main" };
  }

  async function finish() {
    await $backend!.setRoute(id, {
      waypoints: $waypoints,

      name,
      notes,
      infra_type: infraType as InfraType,
      override_infra_type: overrideInfraType,
      tier,
    });
    await autosave();
    $mode = { kind: "main" };
  }

  function cancel() {
    $mode = { kind: "main" };
  }

  async function recalculateSections(
    waypts: Waypoint[],
    overrideInfraType: boolean,
    infraType: string,
  ) {
    try {
      sectionsGj = await $backend!.autosplitRoute(
        id,
        waypts,
        overrideInfraType ? infraType : null,
        tier,
        $majorJunctions,
      );
    } catch (err) {}
  }

  function percentFits(sectionsGj: AutosplitRoute): string {
    let total = 0;
    let fits = 0;
    for (let f of sectionsGj.features) {
      total += f.properties.length;
      if (f.properties.fits) {
        fits += f.properties.length;
      }
    }
    return percent(fits, total);
  }

  function percentHighLoS(sectionsGj: AutosplitRoute): string {
    let total = 0;
    let high = 0;
    for (let f of sectionsGj.features) {
      total += f.properties.length;
      if (f.properties.los == "High") {
        high += f.properties.length;
      }
    }
    return percent(high, total);
  }

  function percentMatchesTier(sectionsGj: AutosplitRoute): string {
    let total = 0;
    let matches = 0;
    for (let f of sectionsGj.features) {
      total += f.properties.length;
      if (f.properties.tier == tier) {
        matches += f.properties.length;
      }
    }
    return percent(matches, total);
  }

  let colorSections = {
    infra_type: constructMatchExpression(
      ["get", "infra_type"],
      infraTypeColors,
      "black",
    ),
    gradient: constructMatchExpression(
      ["get", "gradient_group"],
      gradientColors,
      "black",
    ),
    deliverability: [
      "case",
      ["get", "fits"],
      "green",
      "red",
    ] as DataDrivenPropertyValueSpecification<string>,
    los: constructMatchExpression(
      ["get", "los"],
      levelOfServiceColors,
      "black",
    ),
    tier: constructMatchExpression(["get", "tier"], tierColors, "black"),
  };
</script>

<RouteControls
  {map}
  {finish}
  {cancel}
  {deleteRoute}
  editingExisting={id != null}
  {tier}
>
  <div slot="extra-controls" class="main-controls">
    {#if $waypoints.length >= 2}
      {@const pctFits = percentFits(sectionsGj)}
      {@const pctHighLoS = percentHighLoS(sectionsGj)}
      {@const pctMatchesTier = percentMatchesTier(sectionsGj)}

      <section>
        <h4>
          <!-- svelte-ignore a11y-invalid-attribute -->
          <a
            href="#"
            on:click|preventDefault={() => ($editModeBreakdown = "infra_type")}
            class:focused={$editModeBreakdown == "infra_type"}
          >
            Infrastructure type
          </a>
        </h4>

        <SectionDiagram breakdown="infra_type" {sectionsGj} />

        {#if overrideInfraType}
          <p>
            You've forced this route to always use {infraType}, assuming high
            Level of Service.
          </p>

          <button
            class="ds_button ds_button--secondary"
            on:click={() => (overrideInfraType = false)}
          >
            Remove override
          </button>
        {:else}
          <p>
            The route you've drawn has been split into sections, automatically
            picking an infrastructure type to achieve the best possible Level of
            Service.
          </p>

          <button
            class="ds_button ds_button--secondary"
            on:click={() => {
              overrideInfraType = true;
              showOverrideModal = true;
            }}
          >
            Override infrastructure type
          </button>
        {/if}
      </section>

      <section>
        <!-- svelte-ignore a11y-invalid-attribute -->
        <h4>
          <a
            href="#"
            on:click|preventDefault={() =>
              ($editModeBreakdown = "deliverability")}
            class:focused={$editModeBreakdown == "deliverability"}
          >
            Deliverability
          </a>
        </h4>

        <SectionDiagram breakdown="deliverability" {sectionsGj} />

        {#if pctFits != "100%"}
          <p>
            Only {pctFits} of the route fits in the available streetspace. You may
            need to override the infrastructure type for some sections.
          </p>
        {/if}
      </section>

      <section>
        <h4>
          <!-- svelte-ignore a11y-invalid-attribute -->
          <a
            href="#"
            on:click|preventDefault={() => ($editModeBreakdown = "los")}
            class:focused={$editModeBreakdown == "los"}
          >
            Level of Service
          </a>
        </h4>

        <SectionDiagram breakdown="los" {sectionsGj} />

        {#if pctHighLoS != "100%"}
          <p>
            Only {pctHighLoS} of the route has a high level of service. You may need
            to override the infrastructure type for some sections and reduce traffic
            speeds and volumes.
          </p>
        {/if}
      </section>

      <section>
        <h4>
          <!-- svelte-ignore a11y-invalid-attribute -->
          <a
            href="#"
            on:click|preventDefault={() => ($editModeBreakdown = "tier")}
            class:focused={$editModeBreakdown == "tier"}
          >
            Tier
          </a>
        </h4>

        <SectionDiagram breakdown="tier" {sectionsGj} />

        {#if pctMatchesTier != "100%"}
          <p>
            Only {pctMatchesTier} of the route will use the {tierLabels[tier]} tier.
            Because the route enters and exits settlements, part of it is assigned
            to a different tier.
          </p>
        {/if}
      </section>

      <section>
        <h4>
          <!-- svelte-ignore a11y-invalid-attribute -->
          <a
            href="#"
            on:click|preventDefault={() => ($editModeBreakdown = "gradient")}
            class:focused={$editModeBreakdown == "gradient"}
          >
            Gradient
          </a>
        </h4>

        <SectionDiagram breakdown="gradient" {sectionsGj} />
      </section>

      <h4>Route properties</h4>

      <input
        class="ds_input ds_input--fixed-20"
        placeholder="Name"
        bind:value={name}
      />

      <textarea
        class="ds_input"
        rows="2"
        placeholder="Notes"
        bind:value={notes}
      />

      <div>
        <label>
          Tier:
          <select bind:value={tier}>
            <option value="Primary">Primary routes</option>
            <option value="Secondary">Secondary routes</option>
            <option value="LocalAccess">Local access routes</option>
            <option value="LongDistance">Long distance routes</option>
          </select>
        </label>
      </div>
    {/if}

    <RelevantLayers />

    <LeftSidebarStats />

    <Modal bind:show={showOverrideModal}>
      <PickInfraType
        onFinish={(value) => {
          infraType = value;
          showOverrideModal = false;
        }}
      />
    </Modal>
  </div>

  <span slot="extra-map">
    <GeoJSON data={sectionsGj}>
      <LineLayer
        {...layerId("edit-route-sections")}
        paint={{
          "line-width": 10,
          "line-color": colorSections[$editModeBreakdown],
        }}
      />
    </GeoJSON>
  </span>
</RouteControls>

<style>
  /** TODO These get nested in a strange way**/
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }

  .focused {
    text-decoration: underline;
  }
</style>
