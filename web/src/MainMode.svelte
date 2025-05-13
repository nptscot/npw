<script lang="ts">
  import { BackLink } from "./common";
  import { SplitComponent } from "./common/layout";
  import RelevantLayers from "./layers/RelevantLayers.svelte";
  import LeftSidebarStats from "./stats/LeftSidebarStats.svelte";
  import { currentStage, exitCurrentStage, mode } from "./stores";

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

  // Some stages are unreachable here
  let titles = {
    Primary: "Plan primary network",
    Secondary: "Plan secondary network",
    LocalAccess: "",
    LongDistance: "Plan long distance network",
    assessment: "",
  };
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="controls" class="left">
    <div class="main-controls">
      <header
        class="ds_page-header"
        style="display: flex; justify-content: space-between;"
      >
        <h2 class="ds_page-header__title">{titles[$currentStage]}</h2>
      </header>

      <BackLink
        on:click={() => {
          exitCurrentStage();
          $mode = { kind: "overview" };
        }}
      >
        Back to project overview
      </BackLink>

      {#if $currentStage == "Primary"}
        <p>
          Primary routes form the core of a cycle network. They should closely
          follow the highest demand, most strategic corridors and form key
          connections over LA boundaries.
        </p>

        <div>
          <button
            class="ds_button"
            on:click={() =>
              ($mode = {
                kind: "edit-route",
                id: null,
                anyEdits: false,
                restoreWaypoints: [],
              })}
          >
            Draw new primary route
            <kbd>r</kbd>
          </button>
        </div>
      {:else if $currentStage == "Secondary"}
        <p>
          Secondary tier routes should cover medium cycling demand corridors,
          connect town centres and neighbourhoods. To see severance issues (due
          to high traffic speeds and volumes), turn on the layer on the right.
        </p>

        <div>
          <button
            class="ds_button"
            on:click={() =>
              ($mode = {
                kind: "edit-route",
                id: null,
                anyEdits: false,
                restoreWaypoints: [],
              })}
          >
            Draw new secondary route
            <kbd>r</kbd>
          </button>
        </div>
      {:else if $currentStage == "LongDistance"}
        <p>
          In NPW, long distance routes are primarily inter-settlement links
          connecting settlements together with main urban areas (that are close
          enough for everyday work, shopping or social journeys).
        </p>

        <div>
          <button
            class="ds_button"
            on:click={() =>
              ($mode = {
                kind: "edit-route",
                id: null,
                anyEdits: false,
                restoreWaypoints: [],
              })}
          >
            Draw new long distance route
            <kbd>r</kbd>
          </button>
        </div>
      {/if}

      <div>
        <button
          class="ds_button ds_button--secondary"
          on:click={() => ($mode = { kind: "bulk-edit" })}
        >
          Bulk edit
        </button>
      </div>

      <RelevantLayers />
    </div>

    <LeftSidebarStats />
  </div>
</SplitComponent>

<style>
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
