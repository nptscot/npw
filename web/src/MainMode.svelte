<script lang="ts">
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { mode, routeTool } from "./stores";

  function newRoute() {
    $routeTool!.startRoute();
    $mode = { kind: "sketch-route", id: null };
  }
</script>

<SplitComponent>
  <div slot="top">
    <button on:click={() => ($mode = { kind: "debug" })}>Debug</button>
  </div>
  <div slot="sidebar">
    <h2>Main mode</h2>
    {#if $routeTool}
      <button on:click={newRoute}>New route</button>
    {/if}
  </div>

  <div slot="map"></div>
</SplitComponent>
