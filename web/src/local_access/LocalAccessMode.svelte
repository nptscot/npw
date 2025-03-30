<script lang="ts">
  import { MapEvents } from "svelte-maplibre";
  import { HelpButton } from "../common";
  import { SplitComponent } from "../common/layout";
  import LeftSidebarStats from "../stats/LeftSidebarStats.svelte";
  import { autosave, backend, devMode, mode } from "../stores";
  import Greenspaces from "./Greenspaces.svelte";
  import PointPOIs from "./PointPOIs.svelte";
  import { currentPOI } from "./stores";
  import StreetViewPOI from "./StreetViewPOI.svelte";
  import WarpToPOIs from "./WarpToPOIs.svelte";

  async function fixUnreachable() {
    if ($currentPOI) {
      let input = await $backend!.fixUnreachablePOI(
        $currentPOI.kind,
        $currentPOI.idx,
      );
      await $backend!.setRoute(null, input);
      await autosave();

      // TODO This assumes the fix succeeded. Can we easily check?
      $currentPOI.reachable = true;
    }
  }
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

      {#if $currentPOI}
        <h4>Connect POIs</h4>

        {#if $currentPOI.reachable}
          <p>
            {$currentPOI.description} is connected to the network. The blue path
            shows the route through quiet streets to the network.
          </p>
        {:else}
          <p>
            {$currentPOI.description} is not connected to the network. Enable the
            Reachability layer to see the red severances surrounding it.
          </p>

          <div>
            <button class="ds_button" on:click={fixUnreachable}>
              Add the dashed local access route to connect to the network
            </button>
          </div>

          <div>
            <button
              type="button"
              class="ds_link"
              on:click={() => ($mode = { kind: "edit-route", id: null })}
            >
              Manually draw route instead
            </button>
          </div>
        {/if}
      {/if}

      <WarpToPOIs />
    </div>

    <LeftSidebarStats />
  </div>

  <div slot="map">
    <MapEvents on:click={() => ($currentPOI = null)} />

    <Greenspaces />

    <PointPOIs />

    <StreetViewPOI />
  </div>
</SplitComponent>

<style>
  .main-controls {
    overflow-y: auto;
    padding: 20px;
  }
</style>
