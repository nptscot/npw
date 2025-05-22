<script lang="ts">
  import {
    infraTypeColors,
    infraTypeLabels,
    levelOfServiceColors,
    levelOfServiceLabels,
    stageColors,
    tierColors,
    tierLabels,
  } from "../colors";
  import { BackLink } from "../common";
  import { SplitComponent } from "../common/layout";
  import RelevantLayers from "../layers/RelevantLayers.svelte";
  import Greenspaces from "../local_access/Greenspaces.svelte";
  import PointPOIs from "../local_access/PointPOIs.svelte";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import {
    autosave,
    backend,
    currentStage,
    editsRoadStyle,
    mode,
  } from "../stores";
  import type { Waypoint } from "../types";

  export let ids: number[];
  export let restoreWaypoints: Waypoint[];

  $: headerLabel = { ...tierLabels, assessment: "Assess" }[$currentStage];
  $: labelColor = stageColors[$currentStage];

  async function deleteRoute() {
    if (!window.confirm("Are you sure you want to delete this route?")) {
      return;
    }
    await $backend!.deleteRoutes(ids);
    await autosave();
    $mode = { kind: "main" };
  }

  async function backToDrawing() {
    await $backend!.deleteRoutes(ids);
    await autosave();
    $mode = {
      kind: "edit-route",
      id: null,
      anyEdits: false,
      restoreWaypoints,
    };
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "r") {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT" && tag != "TEXTAREA" && tag != "SELECT") {
        e.preventDefault();
        $mode = {
          kind: "edit-route",
          id: null,
          anyEdits: false,
          restoreWaypoints: [],
        };
      }
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

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

      <BackLink on:click={backToDrawing}>Back to drawing</BackLink>

      <div class="ds_button-group">
        <button class="ds_button" on:click={() => ($mode = { kind: "main" })}>
          Stop drawing
        </button>

        <button class="ds_button ds_button--secondary" on:click={deleteRoute}>
          Delete
        </button>
      </div>

      <div>
        <button
          class="ds_button ds_button--secondary"
          on:click={() =>
            ($mode = {
              kind: "edit-route",
              id: null,
              anyEdits: false,
              restoreWaypoints: [],
            })}
        >
          Draw another route
          <kbd>r</kbd>
        </button>
      </div>

      <p>This route was split into {ids.length} sections.</p>

      {#if $backend}
        {#await $backend.getRouteSections(ids) then sections}
          <table>
            <thead>
              <tr>
                <th>Section</th>
                <th>
                  <button
                    class:selected={$editsRoadStyle == "edits_tier"}
                    on:click={() => ($editsRoadStyle = "edits_tier")}
                  >
                    <i class="fa-solid fa-ranking-star"></i>
                  </button>
                </th>
                <th>
                  <button
                    class:selected={$editsRoadStyle == "edits_infra"}
                    on:click={() => ($editsRoadStyle = "edits_infra")}
                  >
                    <i class="fa-solid fa-road"></i>
                  </button>
                </th>
                <th>
                  <button
                    class:selected={$editsRoadStyle == "edits_deliverability"}
                    on:click={() => ($editsRoadStyle = "edits_deliverability")}
                  >
                    <i class="fa-solid fa-person-digging"></i>
                  </button>
                </th>
                <th>
                  <button
                    class:selected={$editsRoadStyle == "edits_los"}
                    on:click={() => ($editsRoadStyle = "edits_los")}
                  >
                    <i class="fa-solid fa-face-smile"></i>
                  </button>
                </th>
              </tr>
            </thead>
            <tbody>
              {#each sections as section, idx}
                <tr>
                  <td
                    class="section-cell"
                    on:click={() =>
                      ($mode = {
                        kind: "edit-route",
                        id: section.id,
                        anyEdits: false,
                        restoreWaypoints: [],
                      })}
                  >
                    {idx + 1}
                  </td>

                  <td
                    style:background={tierColors[section.tier]}
                    title={"Tier: " + tierLabels[section.tier]}
                  ></td>

                  <td
                    style:background={infraTypeColors[section.infra_type]}
                    title={"Infrastructure type: " +
                      infraTypeLabels[section.infra_type]}
                  ></td>

                  <td
                    style:background={section.fits ? "green" : "red"}
                    title={"Fits: " + (section.fits ? "yes" : "no")}
                  ></td>

                  <td
                    style:background={levelOfServiceColors[section.los]}
                    title={"Level of Service: " +
                      levelOfServiceLabels[section.los]}
                  ></td>
                </tr>
              {/each}
            </tbody>
          </table>
        {/await}
      {/if}

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

  table {
    width: 100%;
  }

  th,
  td {
    border: 1px solid black;
  }

  table button {
    width: 100%;
    border-radius: 0;
    background-color: #fff;
  }

  .selected {
    font-weight: bold;
    background-color: #ccc;
  }

  .section-cell {
    text-align: center;
  }
  .section-cell:hover {
    background: grey;
  }
</style>
