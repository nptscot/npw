<script lang="ts">
  import { QualitativeLegend, SequentialLegend } from "svelte-utils";
  import { subpage } from "../../assess";
  import {
    cnTierColors,
    deprived,
    gradient,
    infraTypeColorLegend,
    levelOfServiceLegend,
    meshDensity,
    nptStreetSpaceColors,
    population,
    reachabilityColors,
    speed,
    streetSpaceColors,
    tierColors,
    traffic,
  } from "../../colors";
  import { Checkbox, HelpButton } from "../../common";
  import {
    backgroundLayer,
    currentStage,
    devMode,
    editModeBreakdown,
    mode,
  } from "../../stores";
  import {
    cyclingDemandHigh,
    cyclingDemandMedium,
    debugCyclingDemandMin,
    debugOriginalData,
    gridMeshDensity,
    styleCyclingDemand,
  } from "../stores";
  import CalculatedRouteNetwork from "./CalculatedRouteNetwork.svelte";

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

  $: directnessNetwork =
    $mode.kind == "main" &&
    $currentStage == "assessment" &&
    $subpage == "directness-network";

  $: anyEnabled =
    !["off", "attractive", "disconnections"].includes($backgroundLayer) ||
    $cyclingDemandHigh ||
    $cyclingDemandMedium ||
    $mode.kind == "edit-route" ||
    $gridMeshDensity ||
    directnessNetwork;
</script>

<div class="panel" class:hidden={!anyEnabled}>
  {#if $backgroundLayer == "cn"}
    <b>Coherent network</b>
    <QualitativeLegend labelColors={cnTierColors} />

    {#if $devMode}
      <Checkbox small bind:checked={$debugOriginalData}>
        Show original data
      </Checkbox>
    {/if}
  {:else if $backgroundLayer == "existing_infra"}
    <b>Existing infrastructure</b>
    <QualitativeLegend labelColors={infraTypeColorLegend} />

    {#if $devMode}
      <Checkbox small bind:checked={$debugOriginalData}>
        Show osmactive data
      </Checkbox>
    {/if}
  {:else if $backgroundLayer == "traffic"}
    <b>Estimated traffic volume</b>
    <SequentialLegend
      colorScale={traffic.colorScale}
      labels={{ limits: traffic.limits }}
    />

    {#if $devMode}
      <Checkbox small bind:checked={$debugOriginalData}>
        Show original data
      </Checkbox>
    {/if}
  {:else if $backgroundLayer == "speed"}
    <b>Estimated speed limit</b>
    <SequentialLegend
      colorScale={speed.colorScale}
      labels={{ buckets: speed.limits }}
    />

    {#if $devMode}
      <Checkbox small bind:checked={$debugOriginalData}>
        Show original data
      </Checkbox>
    {/if}
  {:else if $backgroundLayer == "gradient"}
    <b>Gradient</b>
    <SequentialLegend
      colorScale={gradient.colorScale}
      labels={{ limits: gradient.limits }}
    />
  {:else if $backgroundLayer == "street_space"}
    <b>Street space</b>
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
      <QualitativeLegend labelColors={nptStreetSpaceColors} />
    {:else}
      <QualitativeLegend labelColors={streetSpaceColors} />
    {/if}

    {#if $devMode}
      <Checkbox small bind:checked={$debugOriginalData}>
        Show original data
      </Checkbox>
    {/if}
  {:else if $backgroundLayer == "precalculated_rnet"}
    <b>NPT full network</b>
    <div>
      <label>
        Show demand above:
        <input type="number" bind:value={$debugCyclingDemandMin} />
      </label>
    </div>

    <Checkbox small bind:checked={$styleCyclingDemand}>
      Style based on demand
    </Checkbox>

    {#if $styleCyclingDemand}
      <SequentialLegend
        colorScale={demandColors}
        labels={{ limits: demandLimits }}
      />
    {/if}

    {#if $devMode}
      <Checkbox small bind:checked={$debugOriginalData}>
        Show original data
      </Checkbox>
    {/if}
  {:else if $backgroundLayer == "population"}
    <b>Population</b>
    <SequentialLegend
      colorScale={population.colorScale}
      labels={{ limits: population.limits }}
    />
    <p>
      Darker colours are denser. Only the top 3 densest quintiles are shown.
    </p>
  {:else if $backgroundLayer == "deprived"}
    <b>Deprived population (SIMD)</b>
    <SequentialLegend
      colorScale={deprived.colorScale}
      labels={{ limits: deprived.limits }}
    />
    <p>
      Darker colours are more deprived. Only the top 20%ile most deprived zones
      are shown.
    </p>
  {:else if $backgroundLayer == "los"}
    <b>Level of Service</b>
    <QualitativeLegend labelColors={levelOfServiceLegend} />

    {#if $devMode}
      <Checkbox small bind:checked={$debugOriginalData}>
        Show original data
      </Checkbox>
    {/if}
  {:else if $backgroundLayer == "reachability"}
    <b>Severances</b>
    <QualitativeLegend itemsPerRow={1} labelColors={reachabilityColors} />
    <p>Based on estimated traffic volumes and speeds</p>
  {/if}

  <CalculatedRouteNetwork />

  <!-- TODO: There could be a legend for reference layers, per-tier layers, and
  edit mode all at the same time... -->

  {#if $cyclingDemandHigh || $cyclingDemandMedium}
    <b>Cycling demand</b>

    <Checkbox small bind:checked={$styleCyclingDemand}>
      Style based on demand
    </Checkbox>

    {#if $styleCyclingDemand}
      <SequentialLegend
        colorScale={demandColors}
        labels={{ limits: demandLimits }}
      />
    {/if}
  {/if}

  {#if $mode.kind == "edit-route"}
    {#if $editModeBreakdown == "infra_type"}
      <b>Route: Infrastructure type</b>
      <QualitativeLegend labelColors={infraTypeColorLegend} />
    {:else if $editModeBreakdown == "gradient"}
      <b>Route: Gradient</b>
      <SequentialLegend
        colorScale={gradient.colorScale}
        labels={{ limits: gradient.limits }}
      />
    {:else if $editModeBreakdown == "los"}
      <b>Route: Level of Service</b>
      <QualitativeLegend labelColors={levelOfServiceLegend} />
    {:else if $editModeBreakdown == "tier"}
      <b>Route: Tier</b>
      <QualitativeLegend itemsPerRow={1} labelColors={tierColors} />
    {/if}
  {/if}

  {#if $gridMeshDensity}
    <b>Mesh density</b>
    <SequentialLegend
      colorScale={meshDensity.colorScale}
      labels={{ buckets: meshDensity.legendLimits }}
    />
  {/if}

  {#if directnessNetwork}
    <b>Level of Service</b>
    <QualitativeLegend labelColors={levelOfServiceLegend} />
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
