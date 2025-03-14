<script lang="ts">
  import { QualitativeLegend, SequentialLegend } from "svelte-utils";
  import {
    gradient,
    levelOfServiceColors,
    nptStreetSpaceColors,
    reachabilityColors,
    speed,
    streetSpaceColors,
    traffic,
  } from "../../colors";
  import { HelpButton, percent, prettyPrintDistance } from "../../common";
  import { devMode, referenceRoadStyle, stats } from "../../stores";
  import { debugOriginalData } from "../stores";
  import CalculatedRouteNetwork from "./CalculatedRouteNetwork.svelte";
  import CoreNetwork from "./CoreNetwork.svelte";
  import ExistingNetwork from "./ExistingNetwork.svelte";
  import NetworkDisconnections from "./NetworkDisconnections.svelte";
  import NptFullNetwork from "./NptFullNetwork.svelte";
</script>

<div class="panel">
  <CoreNetwork />

  <ExistingNetwork />

  {#if $referenceRoadStyle == "traffic"}
    <SequentialLegend colorScale={traffic.colorScale} limits={traffic.limits} />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {/if}

  {#if $referenceRoadStyle == "gradient"}
    <SequentialLegend
      colorScale={gradient.colorScale}
      limits={gradient.limits}
    />
  {/if}

  {#if $referenceRoadStyle == "street_space"}
    {#if $debugOriginalData}
      <QualitativeLegend colors={nptStreetSpaceColors} horiz />
    {:else}
      <QualitativeLegend colors={streetSpaceColors} horiz />
    {/if}

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
          You are granted a non-exclusive, royalty free revocable licence solely
          to view the licensed data for non-commercial purposes for the period
          during which Transport Scotland makes it available;
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

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {/if}

  {#if $referenceRoadStyle == "speed"}
    <SequentialLegend colorScale={speed.colorScale} limits={speed.limits} />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {/if}

  <NptFullNetwork />

  {#if $referenceRoadStyle == "los"}
    <QualitativeLegend colors={levelOfServiceColors} horiz />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {/if}

  {#if $referenceRoadStyle == "reachability"}
    <QualitativeLegend colors={reachabilityColors} horiz />
  {/if}

  <CalculatedRouteNetwork />
  <NetworkDisconnections />

  {#if $referenceRoadStyle == "deliverability" && $stats}
    <p>
      {percent($stats.total_undeliverable_length, $stats.total_network_length)} of
      infrastructure ({prettyPrintDistance($stats.total_undeliverable_length)})
      doesn't fit in the available streetspace.
    </p>
  {/if}
</div>

<style>
  .panel {
    position: absolute;
    top: 10px;
    right: calc(10px + 15% + 10px);
    width: 15%;
    min-height: 30px;

    background: white;
  }
</style>
