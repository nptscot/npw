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
  import RoadLayerControls from "./RoadLayerControls.svelte";

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
  <div>
    <CoreNetwork />

    <ExistingNetwork />

    <RoadLayerControls name="Estimated traffic volume" style="traffic">
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
    </RoadLayerControls>

    <RoadLayerControls name="Gradient" style="gradient">
      <SequentialLegend
        colorScale={gradient.colorScale}
        limits={gradient.limits}
      />
    </RoadLayerControls>

    <RoadLayerControls name="Estimated speed limit" style="speed">
      <SequentialLegend colorScale={speed.colorScale} limits={speed.limits} />

      {#if $devMode}
        <label>
          <input type="checkbox" bind:checked={$debugOriginalData} />
          Show original data
        </label>
      {/if}
    </RoadLayerControls>

    <NptFullNetwork />

    <span style:min-width="30px">&nbsp;</span>

    <RoadLayerControls name="Level of Service" style="los">
      <QualitativeLegend colors={levelOfServiceColors} horiz />

      {#if $devMode}
        <label>
          <input type="checkbox" bind:checked={$debugOriginalData} />
          Show original data
        </label>
      {/if}
    </RoadLayerControls>

    <RoadLayerControls name="Reachability" style="reachability">
      <QualitativeLegend colors={reachabilityColors} horiz />
    </RoadLayerControls>

    <CalculatedRouteNetwork />
    <NetworkDisconnections />
  </div>
</Control>

<style>
  div {
    display: flex;
    background: white;
    width: 80%;
  }
</style>
