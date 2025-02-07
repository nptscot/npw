<script lang="ts">
  import { QualitativeLegend } from "svelte-utils";
  import { infraTypeColors, tierColors } from "../../colors";
  import { editsRoadStyle } from "../../stores";
</script>

<div style="border: 2px solid black; padding: 4px">
  <div><b>Your current network edits</b></div>

  <label>
    <input type="radio" value="off" bind:group={$editsRoadStyle} />
    Don't show
  </label>
  <label>
    <input type="radio" value="edits_tier" bind:group={$editsRoadStyle} />
    Tier
  </label>
  <label>
    <input type="radio" value="edits_infra" bind:group={$editsRoadStyle} />
    Infrastructure type
  </label>

  <details>
    <summary>Legend</summary>
    {#if $editsRoadStyle == "edits_infra"}
      <QualitativeLegend colors={infraTypeColors} />
    {:else if $editsRoadStyle == "edits_tier"}
      <QualitativeLegend colors={tierColors} horiz />
    {/if}
  </details>
</div>
