<script lang="ts">
  import type { FeatureCollection } from "geojson";
  import { onMount } from "svelte";
  import { SplitComponent } from "./common/layout";
  import Link from "./common/Link.svelte";
  import ManageFiles from "./common/ManageFiles.svelte";
  import StreetView from "./common/StreetView.svelte";
  import AllControls from "./layers/AllControls.svelte";
  import Stats from "./stats/Stats.svelte";
  import { backend, mode, mutationCounter } from "./stores";

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

  <div slot="map" />

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

    <StreetView />
  </div>
</SplitComponent>
