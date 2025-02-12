<script lang="ts">
  import { Control } from "svelte-maplibre";
  import { QualitativeLegend, SequentialLegend } from "svelte-utils";
  import {
    gradient,
    levelOfServiceColors,
    reachabilityColors,
    speed,
    traffic,
  } from "../../colors";
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

    <div style:display="flex">
      <Toggle name="Core network" style="cn" />
      <Toggle name="Existing infrastructure type" style="existing_infra" />
      <Toggle name="Estimated traffic volume" style="traffic" />
      <Toggle name="Gradient" style="gradient" />
      <Toggle name="Estimated speed limit" style="speed" />
      <Toggle name="NPT full network" style="precalculated_rnet" />
      <span style:min-width="30px">&nbsp;</span>
      <Toggle name="Level of Service" style="los" />
      <Toggle name="Reachability" style="reachability" />
      <Toggle name="Route network (calculated)" style="calculated_rnet" />
      <Toggle name="Network disconnections" style="disconnections" />
    </div>
  </div>
</Control>

<style>
  .panel {
    background: white;
    width: 80%;
  }
</style>
