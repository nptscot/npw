<script lang="ts">
  import { QualitativeLegend } from "svelte-utils";
  import { infraTypeColors } from "../../colors";
  import { backgroundLayer, devMode } from "../../stores";
  import { debugOriginalData } from "../stores";
</script>

{#if $backgroundLayer == "existing_infra"}
  <QualitativeLegend colors={infraTypeColors} />

  {#if $devMode}
    <br />

    <label>
      <input type="checkbox" bind:checked={$debugOriginalData} />
      Show osmactive data
    </label>
  {/if}
{/if}
