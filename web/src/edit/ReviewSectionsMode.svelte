<script lang="ts">
  import { stageColors, tierLabels } from "../colors";
  import { SplitComponent } from "../common/layout";
  import RelevantLayers from "../layers/RelevantLayers.svelte";
  import Greenspaces from "../local_access/Greenspaces.svelte";
  import PointPOIs from "../local_access/PointPOIs.svelte";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import { currentStage, mode } from "../stores";
  import type { AutosplitRoute } from "../types";
  import AllSections from "./AllSections.svelte";

  export let ids: number[];
  export let sectionsGj: AutosplitRoute;

  let tier = $currentStage == "assessment" ? "Primary" : $currentStage;

  $: headerLabel = { ...tierLabels, assessment: "Assess" }[$currentStage];
  $: labelColor = stageColors[$currentStage];
</script>

<SplitComponent>
  <div slot="controls" class="left">
    <div class="main-controls">
      <header class="ds_page-header">
        <span
          class="ds_page-header__label ds_content-label"
          style:color={labelColor}
        >
          {headerLabel}
        </span>

        <h2 class="ds_page-header__title">Review route sections</h2>
      </header>

      <!-- TODO no BackLink? -->

      <button class="ds_button" on:click={() => ($mode = { kind: "main" })}>
        Continue
      </button>

      <p>This route was split into {ids.length} sections.</p>

      <ol>
        {#each ids as id, idx}
          <li>
            <!-- svelte-ignore a11y-invalid-attribute -->
            <a
              href="#"
              on:click|preventDefault={() =>
                ($mode = { kind: "edit-route", id })}
            >
              Section {idx + 1}
            </a>
          </li>
        {/each}
      </ol>

      <AllSections {sectionsGj} {tier} />

      <RelevantLayers />
    </div>

    <LeftSidebarStats />
  </div>

  <div slot="map">
    {#if $currentStage == "LocalAccess"}
      <Greenspaces />
      <PointPOIs />
    {/if}
  </div>
</SplitComponent>

<style>
  /** Controls **/
  .left {
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
  }

  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
