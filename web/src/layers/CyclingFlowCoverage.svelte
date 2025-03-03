<script lang="ts">
  import { devMode } from "../stores";
  import LayerControls from "./LayerControls.svelte";
  import { debugAllCyclingFlow, debugCyclingFlowMin } from "./stores";

  export let show: boolean;
  export let label: string;
</script>

<LayerControls name={label + " cycling flow"} bind:show>
  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={$debugAllCyclingFlow} />
      Debug all flows
    </label>

    {#if $debugAllCyclingFlow}
      <label>
        Show flows above:
        <input type="number" bind:value={$debugCyclingFlowMin} />
      </label>
    {/if}
  {/if}
</LayerControls>
