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

  async function onKeyDown(e: KeyboardEvent) {
    if (e.key == "a" && $currentPOI && !$currentPOI.reachable) {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT") {
        e.preventDefault();
        await fixUnreachable();
      }
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="controls">
    <div class="main-controls">
      {#if !$currentPOI}
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

        <p>
          Your network needs to provide connectivity to key points of interest,
          such as schools, hospitals and green spaces.
        </p>

        <p>
          These places are shown on the map. POIs with severed connectivity are
          shown in red and need to be fixed.
        </p>

        <p>TODO filter</p>

        <p>TODO main button</p>
      {:else}
        <header class="ds_page-header">
          <span class="ds_page-header__label ds_content-label">
            Local Access
          </span>
          <h2 class="ds_page-header__title">Fix connectivity for a POI</h2>
        </header>

        {#if $currentPOI.reachable}
          <p>
            {$currentPOI.description} is connected to the network. The blue path
            shows the route through quiet streets to the network.
          </p>
        {:else}
          <p>
            {$currentPOI.description} is not connected to the network.
          </p>
          <p>A suggested local access route is shown dashed.</p>

          <div>
            <button class="ds_button" on:click={fixUnreachable}>
              Add the dashed line to fix (a)
            </button>
          </div>

          <div>
            <button
              type="button"
              class="ds_link"
              on:click={() => ($mode = { kind: "edit-route", id: null })}
            >
              Or draw a new route line manually
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
