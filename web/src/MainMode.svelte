<script lang="ts">
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { SplitComponent } from "./common/layout";
  import {
    backend,
    mode,
    infraTypeMapping,
    boundaryName,
    mainModeRoutesChanged,
    autosave,
  } from "./stores";
  import { currentNetwork } from "./layers/stores";
  import type { FeatureCollection } from "geojson";
  import { onMount } from "svelte";
  import Link from "./common/Link.svelte";
  import { colorByInfraType } from "./colors";
  import AllControls from "./layers/AllControls.svelte";
  import Stats from "./stats/Stats.svelte";
  import ChangeArea from "./ChangeArea.svelte";

  let gj: FeatureCollection | null = null;
  onMount(recalc);

  $: if ($mainModeRoutesChanged > 0) {
    recalc();
  }

  async function recalc() {
    gj = await $backend!.renderRoutes();
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "r") {
      e.preventDefault();
      $mode = { kind: "edit-route", id: null };
    }
  }

  function editRouteMap(e: CustomEvent<LayerClickInfo>) {
    $mode = { kind: "edit-route", id: e.detail.features[0].id as number };
  }

  function editRouteSidebar(id: string | number | undefined) {
    $mode = { kind: "edit-route", id: id as number };
  }

  async function clearAll() {
    if (window.confirm("Clear everything? You can't undo this")) {
      await $backend?.clearAllRoutes();
      await autosave();
      await recalc();
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="left">
    <ChangeArea area={$boundaryName} />

    <div>
      <button on:click={() => ($mode = { kind: "edit-route", id: null })}>
        Draw new <u>r</u>
        oute line
      </button>

      <button class="secondary" on:click={clearAll}>Clear all</button>
    </div>

    <label>
      <input type="checkbox" bind:checked={$currentNetwork} />
      Show current network
    </label>
    <details>
      <summary>Current network routes</summary>

      {#if gj}
        <ol>
          {#each gj.features as f}
            <li>
              <Link on:click={() => editRouteSidebar(f.id)}>
                {f.properties?.name || `Untitled route ${f.id}`} ({f.properties
                  ?.infra_type})
              </Link>
            </li>
          {/each}
        </ol>
      {/if}
    </details>

    <hr />

    <AllControls />
  </div>

  <div slot="map">
    {#if gj}
      <GeoJSON data={gj}>
        <LineLayer
          id="routes"
          paint={{
            "line-width": hoverStateFilter(5, 7),
            "line-color": colorByInfraType,
          }}
          layout={{
            visibility: $currentNetwork ? "visible" : "none",
          }}
          manageHoverState
          hoverCursor="pointer"
          on:click={editRouteMap}
        >
          <Popup openOn="hover" let:props>
            {props.name || "Untitled"} ({infraTypeMapping[props.infra_type][0]})
          </Popup>
        </LineLayer>
      </GeoJSON>
    {/if}
  </div>

  <div slot="right">
    <h4>Evaluate:</h4>

    <Stats />

    <div>
      <button
        class="secondary"
        on:click={() => ($mode = { kind: "evaluate-route" })}
      >
        Evaluate a route
      </button>
    </div>

    <div>
      <button
        class="secondary"
        on:click={() => ($mode = { kind: "evaluate-od" })}
      >
        Evaluate OD
      </button>
    </div>

    <div>
      <button
        class="secondary"
        on:click={() => ($mode = { kind: "debug-network" })}
      >
        Debug network
      </button>
    </div>

    <div>
      <button
        class="secondary"
        on:click={() => ($mode = { kind: "debug-mesh-density" })}
      >
        Debug mesh density
      </button>
    </div>
  </div>
</SplitComponent>
