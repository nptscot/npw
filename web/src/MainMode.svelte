<script lang="ts">
  import { SplitComponent } from "./common/layout";
  import AllControls from "./layers/AllControls.svelte";
  import LeftSidebarStats from "./stats/LeftSidebarStats.svelte";
  import { currentStage, mode } from "./stores";

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
  <div slot="left">
    <h2>{titles[$currentStage]}</h2>

    <div>
      <button on:click={() => ($mode = { kind: "edit-route", id: null })}>
        Draw new <kbd>r</kbd>
        oute line
      </button>
    </div>
    <div>
      <button class="outline" on:click={() => ($mode = { kind: "bulk-edit" })}>
        Bulk edit
      </button>
    </div>
    <div>
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
    </div>

    <AllControls />

    <LeftSidebarStats />
  </div>
</SplitComponent>
