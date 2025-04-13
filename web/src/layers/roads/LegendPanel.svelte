<script lang="ts">
  import { QualitativeLegend, SequentialLegend } from "svelte-utils";
  import {
    cnTierColors,
    deprived,
    gradient,
    gradientColors,
    infraTypeColorLegend,
    levelOfServiceLabels,
    meshDensity,
    nptStreetSpaceColors,
    population,
    reachabilityColors,
    speed,
    streetSpaceColors,
    tierColors,
    traffic,
  } from "../../colors";
  import { HelpButton } from "../../common";
  import {
    backgroundLayer,
    devMode,
    editModeBreakdown,
    mode,
  } from "../../stores";
  import {
    cyclingDemandHigh,
    cyclingDemandMedium,
    debugOriginalData,
    gridMeshDensity,
  } from "../stores";
  import CalculatedRouteNetwork from "./CalculatedRouteNetwork.svelte";
  import NptFullNetwork from "./NptFullNetwork.svelte";

  // common has functions ForDemand, duplicating values here
  // TODO Center on the buckets instad
  let demandLimits = [1, 50, 100, 250, 500, 1000, 2000, 3000];
  let demandColors = [
    "#9C9C9C",
    "#FFFF73",
    "#AFFF00",
    "#00FFFF",
    "#30B0FF",
    "#2E5FFF",
    "#0000FF",
    "#FF00C5",
  ];

  $: anyEnabled =
    !["off", "attractive", "disconnections"].includes($backgroundLayer) ||
    $cyclingDemandHigh ||
    $cyclingDemandMedium ||
    $mode.kind == "edit-route" ||
    $gridMeshDensity;
</script>

<div class="panel" class:hidden={!anyEnabled}>
  {#if $backgroundLayer == "cn"}
    <QualitativeLegend colors={cnTierColors} />

    {#if $devMode}
      <br />

      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $backgroundLayer == "existing_infra"}
    <QualitativeLegend colors={infraTypeColorLegend} />

    {#if $devMode}
      <br />

      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show osmactive data
      </label>
    {/if}
  {:else if $backgroundLayer == "traffic"}
    <SequentialLegend colorScale={traffic.colorScale} limits={traffic.limits} />

    {#if $devMode}
      <br />

      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $backgroundLayer == "gradient"}
    <SequentialLegend
      colorScale={gradient.colorScale}
      limits={gradient.limits}
    />
  {:else if $backgroundLayer == "street_space"}
    <p>
      What fits within the carriageway, verges, and footways?
      <HelpButton>
        <p>
          Contains <a
            href="https://www.ordnancesurvey.co.uk/customers/public-sector/public-sector-licensing/copyright-acknowledgments"
            target="_blank"
          >
            OS data
          </a>
          &copy; Crown copyright and database rights 2025 OS licence number 100046668.
        </p>
        <ul>
          <li>
            You are granted a non-exclusive, royalty free revocable licence
            solely to view the licensed data for non-commercial purposes for the
            period during which Transport Scotland makes it available;
          </li>
          <li>
            You are not permitted to copy, sub-license, distribute, sell or
            otherwise make available the licensed data to third parties in any
            form; and
          </li>
          <li>
            Third party rights to enforce the terms of this licence shall be
            reserved to OS.
          </li>
        </ul>
      </HelpButton>
    </p>

    {#if $debugOriginalData}
      <QualitativeLegend colors={nptStreetSpaceColors} />
    {:else}
      <QualitativeLegend colors={streetSpaceColors} />
    {/if}

    {#if $devMode}
      <br />

      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $backgroundLayer == "speed"}
    <SequentialLegend colorScale={speed.colorScale} limits={speed.limits} />

    {#if $devMode}
      <br />

      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $backgroundLayer == "los"}
    <QualitativeLegend colors={levelOfServiceLabels} />

    {#if $devMode}
      <br />

      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $backgroundLayer == "reachability"}
    <QualitativeLegend colors={reachabilityColors} />
  {:else if $backgroundLayer == "deprived"}
    <SequentialLegend
      colorScale={deprived.colorScale}
      limits={deprived.limits}
    />
    <p>
      Darker colours are more deprived. Only the top 20%ile most deprived zones
      are shown.
    </p>
  {:else if $backgroundLayer == "population"}
    <SequentialLegend
      colorScale={population.colorScale}
      limits={population.limits}
    />
    <p>
      Darker colours are denser. Only the top 3 densest quintiles are shown.
    </p>
  {/if}

  <NptFullNetwork />
  <CalculatedRouteNetwork />

  <!-- TODO: There could be a legend for reference layers, per-tier layers, and
  edit mode all at the same time... -->

  {#if $cyclingDemandHigh || $cyclingDemandMedium}
    <SequentialLegend colorScale={demandColors} limits={demandLimits} />
  {/if}

  {#if $mode.kind == "edit-route"}
    {#if $editModeBreakdown == "infra_type"}
      <QualitativeLegend colors={infraTypeColorLegend} />
    {:else if $editModeBreakdown == "gradient"}
      <QualitativeLegend colors={gradientColors} />
    {:else if $editModeBreakdown == "los"}
      <QualitativeLegend colors={levelOfServiceLabels} />
    {:else if $editModeBreakdown == "tier"}
      <QualitativeLegend colors={tierColors} />
    {/if}
  {/if}

  {#if $gridMeshDensity}
    <SequentialLegend
      colorScale={meshDensity.colorScale}
      limits={meshDensity.legendLimits}
    />
  {/if}
</div>

<style>
  .panel {
    position: absolute;
    top: 10px;
    right: 200px;
    width: 200px;

    background: white;
    padding: 8px;
  }

  .hidden {
    display: none;
  }
</style>
