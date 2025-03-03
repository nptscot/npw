<script lang="ts">
  import { SplitComponent } from "./common/layout";
  import AllControls from "./layers/AllControls.svelte";
  import LeftSidebarStats from "./stats/LeftSidebarStats.svelte";
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
          kind: "evaluate-journey",
          prevMode: { kind: "main" },
          browse: [],
        })}
    >
      Evaluate a journey
    </button>

    <AllControls />

    <LeftSidebarStats />
  </div>
</SplitComponent>
