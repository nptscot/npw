<script lang="ts">
  import { HelpButton } from "../common";
  import { SplitComponent } from "../common/layout";
  import RelevantLayers from "../layers/RelevantLayers.svelte";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import { devMode, mode } from "../stores";
  import Greenspaces from "./Greenspaces.svelte";
  import POIs from "./POIs.svelte";
</script>

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      <header
        class="ds_page-header"
        style="display: flex; justify-content: space-between;"
      >
        <h2 class="ds_page-header__title">Design local access network</h2>

        <HelpButton>
          <p>
            To draw the local access route, connect schools, GPs, hospitals,
            green spaces, and neighbourhoods (especially deprived and densely
            populated ones).
          </p>

          <label>
            <input type="checkbox" bind:checked={$devMode} />
            Dev mode
          </label>
        </HelpButton>
      </header>

      <RelevantLayers />

      <br />

      <div>
        <button
          class="ds_button"
          on:click={() => ($mode = { kind: "edit-route", id: null })}
        >
          Draw new route line
        </button>
      </div>
    </div>

    <LeftSidebarStats />
  </div>

  <div slot="map">
    <Greenspaces />

    <POIs />
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
