<script lang="ts">
  import { SplitComponent } from "svelte-utils/top_bar_layout";
  import { mode, routeTool } from "./stores";
  import { onDestroy } from "svelte";
  import RouteSnapperLayer from "./snapper/RouteSnapperLayer.svelte";
  import RouteControls from "./snapper/RouteControls.svelte";

  export let id: number | null;

  // The user can change the mode in many ways, like clicking a link.
  // When this component gets destroyed, always clean up state.
  onDestroy(() => {
    $routeTool?.clearEventListeners();
    $routeTool?.stop();
  });

  $routeTool!.addEventListenerSuccess((feature) => {
    window.alert(
      `got a route with ${feature.properties.full_path.length} nodes`,
    );
    $mode = {
      kind: "main",
    };
  });
  $routeTool!.addEventListenerFailure(() => {
    $mode = {
      kind: "main",
    };
  });
</script>

<SplitComponent>
  <div slot="top">Nav</div>
  <div slot="sidebar">
    <h2>Sketch route mode</h2>
    {#if $routeTool}
      <RouteControls routeTool={$routeTool} />
    {/if}
  </div>

  <div slot="map">
    <RouteSnapperLayer />
  </div>
</SplitComponent>
