<script lang="ts">
  import { QualitativeLegend } from "svelte-utils";
  import { cnTierColors } from "../../colors";
  import { backgroundLayer, devMode } from "../../stores";
  import { debugOriginalData } from "../stores";
</script>

{#if $backgroundLayer == "cn"}
  <QualitativeLegend colors={cnTierColors} />

  {#if $devMode}
    <br />

    <label>
      <input type="checkbox" bind:checked={$debugOriginalData} />
      Show original data
    </label>
  {/if}
{/if}
