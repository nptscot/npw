<script lang="ts">
  import { QualitativeLegend } from "svelte-utils";
  import {
    infraTypeColors,
    infraTypeLabels,
    levelOfServiceColors,
    levelOfServiceLabels,
    tierColors,
    tierLabels,
  } from "../../colors";
  import { LegendWithToggles } from "../../common";
  import { editsRoadStyle } from "../../stores";
  import {
    showNetworkDeliverability,
    showNetworkInfraTypes,
    showNetworkLoS,
    showNetworkTiers,
  } from "../stores";
</script>

<div class="network-style">
  <div style="display: flex">
    <ul style:flex-grow="1">
      <li class:selected={$editsRoadStyle == "off"}>
        <button on:click={() => ($editsRoadStyle = "off")}>
          &#10680;&nbsp; Hidden
        </button>
      </li>

      <li class:selected={$editsRoadStyle == "edits_tier"}>
        <button on:click={() => ($editsRoadStyle = "edits_tier")}>
          ‖‖&nbsp; Tier
        </button>
      </li>

      <li class:selected={$editsRoadStyle == "edits_infra"}>
        <button on:click={() => ($editsRoadStyle = "edits_infra")}>
          ☰&nbsp; Infrastructure type
        </button>
      </li>

      <li class:selected={$editsRoadStyle == "edits_deliverability"}>
        <button on:click={() => ($editsRoadStyle = "edits_deliverability")}>
          <i class="fa-solid fa-person-digging"></i>
          Deliverability
        </button>
      </li>

      <li class:selected={$editsRoadStyle == "edits_los"}>
        <button on:click={() => ($editsRoadStyle = "edits_los")}>
          <i class="fa-solid fa-face-smile"></i>
          Level of Service
        </button>
      </li>
    </ul>
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
  {:else if $editsRoadStyle == "edits_deliverability"}
    <LegendWithToggles
      labels={{ deliverable: "Deliverable", not: "Not enough space" }}
      colors={{ deliverable: "green", not: "red" }}
      show={showNetworkDeliverability}
    />
  {:else if $editsRoadStyle == "edits_los"}
    <LegendWithToggles
      labels={levelOfServiceLabels}
      colors={levelOfServiceColors}
      show={showNetworkLoS}
    />
  {:else if $editsRoadStyle == "off"}
    <!-- Just maintain the vertical space -->
    <span style:visibility="hidden">
      <QualitativeLegend colors={{ hidden: "black" }} horiz />
    </span>
  {/if}
</div>

<style>
  .network-style {
    position: absolute;
    bottom: 10px;
    width: 610px;
    left: 0;
    right: 0;
    margin-inline: auto;
    background-color: white;
    padding: 10px;
    font-size: 14px;
  }
  .network-style :global(ul) {
    margin: 0;
  }
  .network-style :global(ul) {
    display: flex;
    flex-direction: row;
    margin: 0;
  }
  .network-style :global(ul li) {
    flex-grow: 1;
    flex-basis: 0;
    list-style: none;
  }
  .network-style :global(ul li:last-child) {
    margin-bottom: auto;
  }
  .network-style ul li button {
    width: 100%;
    border-radius: 0;
    border: 1px solid gray;
    background-color: #eee;
    text-align: left;
  }
  .network-style ul li.selected button {
    font-weight: bold;
    background-color: #ccc;
  }
  .network-style :global(ul li label) {
    display: block;
    border-radius: 0;
    padding: 2px 6px;
    text-align: left;
  }
  .network-style :global(ul li:hover) {
    color: black;
    opacity: 0.9;
  }
</style>
