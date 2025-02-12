<script lang="ts">
  import { QualitativeLegend } from "svelte-utils";
  import { infraTypeColors, tierColors } from "../../colors";
  import { editsRoadStyle } from "../../stores";
</script>

<div style="border: 2px solid black; padding: 4px">
  <b>Show your new network as:</b>
  <div style:display="flex">
    <button
      style:font-size="12px"
      style:background={$editsRoadStyle == "edits_tier" ? "green" : "grey"}
      on:click={() => ($editsRoadStyle = "edits_tier")}
    >
      Tier
    </button>
    <button
      style:font-size="12px"
      style:background={$editsRoadStyle == "edits_infra" ? "green" : "grey"}
      on:click={() => ($editsRoadStyle = "edits_infra")}
    >
      Infrastructure type
    </button>
    <button
      style:font-size="12px"
      style:background={$editsRoadStyle == "off" ? "green" : "grey"}
      on:click={() => ($editsRoadStyle = "off")}
    >
      Hidden
    </button>
  </div>

  <details>
    <summary>Legend</summary>
    {#if $editsRoadStyle == "edits_infra"}
      <QualitativeLegend colors={infraTypeColors} />
    {:else if $editsRoadStyle == "edits_tier"}
      <QualitativeLegend colors={tierColors} horiz />
    {/if}
  </details>
</div>
