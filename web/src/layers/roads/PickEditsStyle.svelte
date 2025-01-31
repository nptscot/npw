<script lang="ts">
  import { infraTypeColors, tierColors } from "../../colors";
  import { QualitativeLegend } from "../../common";
  import { editsRoadStyle } from "../../stores";
</script>

<label>
  Show your current network edits:
  <select bind:value={$editsRoadStyle}>
    <option value="off">Don't show</option>
    <option value="edits_infra">By infrastructure type</option>
    <option value="edits_tier">By tier</option>
  </select>
</label>

<details>
  <summary>Legend</summary>
  {#if $editsRoadStyle == "edits_infra"}
    <QualitativeLegend colors={infraTypeColors} />
  {:else if $editsRoadStyle == "edits_tier"}
    <QualitativeLegend colors={tierColors} horiz />
  {/if}
</details>
