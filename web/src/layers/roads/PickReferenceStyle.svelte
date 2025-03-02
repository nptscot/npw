<script lang="ts">
  import { Control } from "svelte-maplibre";
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
  import { HelpButton } from "../../common";
  import { devMode, referenceRoadStyle } from "../../stores";
  import { debugOriginalData, lastReferenceStyle } from "../stores";
  import CalculatedRouteNetwork from "./CalculatedRouteNetwork.svelte";
  import CoreNetwork from "./CoreNetwork.svelte";
  import ExistingNetwork from "./ExistingNetwork.svelte";
  import NetworkDisconnections from "./NetworkDisconnections.svelte";
  import NptFullNetwork from "./NptFullNetwork.svelte";
  import Toggle from "./Toggle.svelte";

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "s") {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT") {
        e.preventDefault();

        if ($referenceRoadStyle == "off") {
          $referenceRoadStyle = $lastReferenceStyle;
        } else {
          $lastReferenceStyle = $referenceRoadStyle;
          $referenceRoadStyle = "off";
        }
      }
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<Control position="bottom-left">
  <div class="panel">
    <CoreNetwork />

    <ExistingNetwork />

    {#if $referenceRoadStyle == "traffic"}
      <SequentialLegend
        colorScale={traffic.colorScale}
        limits={traffic.limits}
      />

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

    {#if $referenceRoadStyle == "deliverability"}
      <p>TODO of infrastructure doesn't fit in the available streetspace</p>
    {/if}

    <div style:display="flex">
      <Toggle name="Core network" style="cn" />
      <Toggle name="Existing infrastructure type" style="existing_infra" />
      <Toggle name="Estimated traffic volume" style="traffic" />
      <Toggle name="Gradient" style="gradient" />
      <Toggle name="Street space" style="street_space" />
      <Toggle name="Estimated speed limit" style="speed" />
      <Toggle name="NPT full network" style="precalculated_rnet" />
      <span style:min-width="30px">&nbsp;</span>
      <Toggle name="Level of Service" style="los" />
      <Toggle name="Reachability" style="reachability" />
      <Toggle name="Route network (calculated)" style="calculated_rnet" />
      <Toggle name="Network disconnections" style="disconnections" />
      <Toggle name="Streetspace deliverability" style="deliverability" />
    </div>
  </div>
</Control>

<style>
  .panel {
    background: white;
    width: 80%;
  }
</style>
