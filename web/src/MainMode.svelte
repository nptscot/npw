<script lang="ts">
  import { HelpButton } from "./common";
  import { SplitComponent } from "./common/layout";
  import RelevantLayers from "./layers/RelevantLayers.svelte";
  import LeftSidebarStats from "./stats/LeftSidebarStats.svelte";
  import { currentStage, devMode, mode } from "./stores";

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "r") {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT") {
        e.preventDefault();
        $mode = { kind: "edit-route", id: null };
      }
    }
  }

  // Some stages are unreachable here
  let titles = {
    Primary: "Design primary network",
    Secondary: "Design secondary network",
    LocalAccess: "",
    LongDistance: "Design long distance network",
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

        <HelpButton>
          {#if $currentStage == "Primary"}
            <p>
              The primary route should be direct, coherent, meet high demand,
              and potentially connect settlements. Key primary routes will form
              Active Freeways. To draw the primary route, connect the high
              cycling demand routes on the base map.
            </p>
          {:else if $currentStage == "Secondary"}
            <p>
              To draw the secondary route, please connect town centres and cover
              medium cycling demand routes on the base map.
            </p>
          {:else if $currentStage == "LongDistance"}
            <ul>
              <li>
                Long distance routes connect EDJ reachable settlements out with
                main urban areas.
              </li>
              <li>
                Settlements should be connected by high demand routes forming a
                direct connection in most cases.
              </li>
              <li>
                Long distance routes should connect directly to primary routes
                within each settlement.
              </li>
              <li>Consider SIMD/transport poverty</li>
              <li>
                In limited circumstances, settlement can be connected by less
                direct/more scenic routes (NCN)
              </li>
            </ul>
          {/if}

          <label>
            <input type="checkbox" bind:checked={$devMode} />
            Dev mode
          </label>
        </HelpButton>
      </header>

      <div>
        <button
          type="button"
          class="ds_link"
          on:click={() => ($mode = { kind: "overview" })}
        >
          <i class="fa-solid fa-chevron-left"></i>
          Back to project overview
        </button>
      </div>

      <div>
        <button
          class="ds_button"
          on:click={() => ($mode = { kind: "edit-route", id: null })}
        >
          Draw new route line (
          <kbd>r</kbd>
          )
        </button>
      </div>
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
