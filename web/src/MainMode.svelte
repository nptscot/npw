<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { onMount } from "svelte";
  import {
    GeoJSON,
    hoverStateFilter,
    LineLayer,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import { colorByInfraType, colorByTier } from "./colors";
  import { layerId } from "./common";
  import { SplitComponent } from "./common/layout";
  import Link from "./common/Link.svelte";
  import ManageFiles from "./common/ManageFiles.svelte";
  import StreetView from "./common/StreetView.svelte";
  import AllControls from "./layers/AllControls.svelte";
  import { currentNetwork } from "./layers/stores";
  import Stats from "./stats/Stats.svelte";
  import {
    backend,
    colorRoutesBy,
    infraTypeMapping,
    mode,
    mutationCounter,
    tier,
  } from "./stores";

  let gj: FeatureCollection | null = null;
  onMount(recalc);

  $: if ($mutationCounter > 0) {
    recalc();
  }

  async function recalc() {
    gj = await $backend!.renderRoutes();
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "r") {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT") {
        e.preventDefault();
        $mode = { kind: "edit-route", id: null };
      }
    }
  }

  function editRouteMap(e: CustomEvent<LayerClickInfo>) {
    $mode = { kind: "edit-route", id: e.detail.features[0].id as number };
  }

  function editRouteSidebar(id: string | number | undefined) {
    $mode = { kind: "edit-route", id: id as number };
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="left">
    <a href="index.html">Change area</a>
    <ManageFiles />

    <button on:click={() => ($mode = { kind: "edit-route", id: null })}>
      Draw new <u>r</u>
      oute line
    </button>

    <label>
      <input type="checkbox" bind:checked={$currentNetwork} />
      Show current network
    </label>
    <details>
      <summary>Current network routes</summary>

      <label>
        Show routes by:
        <select bind:value={$colorRoutesBy}>
          <option value="infra_type">Infrastructure type</option>
          <option value="tier">Tier</option>
        </select>
      </label>

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
          {...layerId("main-mode")}
          paint={{
            "line-width": [
              "case",
              ["==", ["get", "tier"], $tier],
              hoverStateFilter(5, 7),
              hoverStateFilter(3, 5),
            ],
            "line-color":
              $colorRoutesBy == "infra_type" ? colorByInfraType : colorByTier,
          }}
          layout={{
            visibility: $currentNetwork ? "visible" : "none",
          }}
          manageHoverState
          hoverCursor="pointer"
          on:click={editRouteMap}
        >
          <Popup openOn="hover" let:props>
            {props.name || "Untitled"} ({infraTypeMapping[props.infra_type][0]},
            {props.tier})
          </Popup>
        </LineLayer>
      </GeoJSON>
    {/if}
  </div>

  <div slot="right">
    <Stats />

    <div>
      <button
        class="secondary"
        on:click={() =>
          ($mode = {
            kind: "evaluate-route",
            prevMode: { kind: "main" },
            browse: [],
          })}
      >
        Evaluate a route
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

    <StreetView />
  </div>
</SplitComponent>
