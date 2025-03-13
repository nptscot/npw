<script lang="ts">
  import { QualitativeLegend } from "svelte-utils";
  import {
    infraTypeColors,
    infraTypeLabels,
    tierColors,
    tierLabels,
  } from "../../colors";
  import { LegendWithToggles } from "../../common";
  import { editsRoadStyle } from "../../stores";
  import { showNetworkInfraTypes, showNetworkTiers } from "../stores";
</script>

<div class="bottom-center">
  <b>Show your new network as</b>
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

  {#if $editsRoadStyle == "edits_infra"}
    <LegendWithToggles
      labels={infraTypeLabels}
      colors={infraTypeColors}
      show={showNetworkInfraTypes}
    />
  {:else if $editsRoadStyle == "edits_tier"}
    <LegendWithToggles
      labels={tierLabels}
      colors={tierColors}
      show={showNetworkTiers}
    />
  {:else if $editsRoadStyle == "off"}
    <!-- Just maintain the vertical space -->
    <span style:visibility="hidden">
      <QualitativeLegend colors={{ hidden: "black" }} horiz />
    </span>
  {/if}
</div>

<style>
  .bottom-center {
    position: absolute;
    bottom: 10px;
    width: 35vw;
    left: 50%;
    transform: translateX(-50%);

    background: white;
    padding: 4px;
  }
</style>
