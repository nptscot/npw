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

  let titles = {
    Primary: "Design primary network",
    Secondary: "Design secondary network",
    LocalAccess: "Design local access network",
    LongDistance: "Design long distance network",
    assessment: "Assess the new network",
  };
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="controls">
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
          {:else if $currentStage == "LocalAccess"}
            <p>
              To draw the local access route, connect schools, GPs, hospitals,
              green spaces, and neighbourhoods (especially deprived and densely
              populated ones).
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
          {:else if $currentStage == "assessment"}
            <p>
              Having designed your network, you can now assess its performance
              and fix any problems.
            </p>
          {/if}

          <label>
            <input type="checkbox" bind:checked={$devMode} />
            Dev mode
          </label>
        </HelpButton>
      </header>

      {#if $currentStage == "LocalAccess"}
        <RelevantLayers />
        <br />
      {/if}

      <div>
        <button
          class="ds_button"
          on:click={() => ($mode = { kind: "edit-route", id: null })}
        >
          Draw new route line
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
      <div>
        <button
          class="ds_button ds_button--secondary"
          on:click={() =>
            ($mode = {
              kind: "evaluate-journey",
              prevMode: { kind: "main" },
              browse: [],
            })}
        >
          Evaluate a journey
        </button>
      </div>

      {#if $currentStage != "LocalAccess"}
        <RelevantLayers />
      {/if}
    </div>

    <LeftSidebarStats />
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
