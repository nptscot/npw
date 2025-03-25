<script lang="ts">
  import type {
    DataDrivenPropertyValueSpecification,
    ExpressionSpecification,
  } from "maplibre-gl";
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { constructMatchExpression, emptyGeojson } from "svelte-utils/map";
  import {
    gradientColors,
    infraTypeColors,
    levelOfServiceColors,
  } from "../colors";
  import { layerId, Modal, percent } from "../common";
  import { SplitComponent } from "../common/layout";
  import RelevantLayers from "../layers/RelevantLayers.svelte";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import {
    autosave,
    backend,
    currentStage,
    editModeBreakdown,
    mode,
  } from "../stores";
  import type { AutosplitRoute, Waypoint } from "../types";
  import PickInfraType from "./PickInfraType.svelte";
  import SectionDiagram from "./SectionDiagram.svelte";

  // TODO Maybe as an intermediate, we take waypoints, not ids. maybe that's it.
  export let ids: number[];

  let name = "";
  let notes = "";
  // This is not meaningful when overrideInfraType is false
  let infraType = "MixedTraffic";
  let overrideInfraType = false;
  let tier = $currentStage == "assessment" ? "Primary" : $currentStage;
  let waypoints: Waypoint[] = [];

  let showOverrideModal = false;

  let sectionsGj: AutosplitRoute = emptyGeojson() as AutosplitRoute;
  $: recalculateSections(waypoints, overrideInfraType, infraType);
  $: pctFits = percentFits(sectionsGj);
  $: pctHighLoS = percentHighLoS(sectionsGj);

  onMount(async () => {
    // TODO not sure how this works
    let feature = await $backend!.getRoute(ids[0]);
    name = feature.properties.name;
    notes = feature.properties.notes;
    infraType = feature.properties.infra_type;
    overrideInfraType = feature.properties.override_infra_type;
    tier = feature.properties.tier;
    waypoints = feature.properties.waypoints;
  });

  async function deleteRoutes() {
    await $backend!.deleteRoutes(ids);
    await autosave();
    $mode = { kind: "main" };
  }

  async function save() {
    try {
      // TODO changeTier, changeInfraType, etc

      /*await $backend!.setRoute(id, {
        feature,
        roads: feature.properties.roads,

        name,
        notes,
        infra_type: infraType,
        override_infra_type: overrideInfraType,
        tier,
      });*/
      window.alert("TODO save this stuff");
      await autosave();
      $mode = { kind: "main" };
    } catch (err) {
      window.alert(err);
    }
  }

  function cancel() {
    $mode = { kind: "main" };
  }

  async function recalculateSections(
    waypts: Waypoint[],
    overrideInfraType: boolean,
    infraType: string,
  ) {
    sectionsGj = emptyGeojson() as AutosplitRoute;

    try {
      let feature = await $backend!.snapRoute(waypts);
      sectionsGj = await $backend!.autosplitRoute(
        ids,
        feature.properties.roads,
        overrideInfraType ? infraType : null,
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

  let filterSections = {
    infra_type: ["==", ["get", "kind"], "new"] as ExpressionSpecification,
    gradient: undefined,
    deliverability: undefined,
    los: undefined,
  };

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
  };
</script>

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      <header class="ds_page-header">
        <h2 class="ds_page-header__title">Edit a route</h2>
      </header>

      <div>
        <button class="ds_button" on:click={save}>Finish editing</button>
      </div>
      <div>
        <button class="ds_button ds_button--secondary" on:click={cancel}>
          Cancel
        </button>
      </div>
      <div>
        <button class="ds_button ds_button--secondary" on:click={deleteRoutes}>
          Delete
        </button>
      </div>

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

      <RelevantLayers />

      <LeftSidebarStats />
    </div>

    <Modal bind:show={showOverrideModal}>
      <PickInfraType bind:current={infraType} />
      <button class="ds_button" on:click={() => (showOverrideModal = false)}>
        OK
      </button>
    </Modal>
  </div>

  <div slot="map">
    <GeoJSON data={sectionsGj}>
      <LineLayer
        {...layerId("edit-route-sections")}
        filter={filterSections[$editModeBreakdown]}
        paint={{
          "line-width": 10,
          "line-color": colorSections[$editModeBreakdown],
        }}
      />
    </GeoJSON>
  </div>
</SplitComponent>

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
