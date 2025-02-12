<script lang="ts">
  import { SplitComponent } from "./common/layout";
  import ManageFiles from "./common/ManageFiles.svelte";
  import AllControls from "./layers/AllControls.svelte";
  import Stats from "./stats/Stats.svelte";
  import { mode } from "./stores";

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "r") {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT") {
        e.preventDefault();
        $mode = { kind: "edit-route", id: null };
      }
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<SplitComponent>
  <div slot="left">
    <a href="index.html">Change area</a>
    <ManageFiles />

    <button on:click={() => ($mode = { kind: "edit-route", id: null })}>
      Draw new <kbd>r</kbd>
      oute line
    </button>
    <button class="outline" on:click={() => ($mode = { kind: "bulk-edit" })}>
      Bulk edit
    </button>
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

    <AllControls />
  </div>

  <div slot="map" />

  <div slot="right">
    <Stats />
  </div>
</SplitComponent>
