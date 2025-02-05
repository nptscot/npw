<script lang="ts">
  import { SequentialLegend } from "svelte-utils";
  import {
    gradient,
    levelOfServiceColors,
    reachabilityColors,
    speed,
    traffic,
  } from "../../colors";
  import { QualitativeLegend } from "../../common";
  import { devMode, referenceRoadStyle } from "../../stores";
  import RoadLayerControls from "../RoadLayerControls.svelte";
  import { debugOriginalData, lastReferenceStyle } from "../stores";
  import CoreNetwork from "./CoreNetwork.svelte";
  import ExistingNetwork from "./ExistingNetwork.svelte";

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

<RoadLayerControls name="Don't show" style="off" empty />

<CoreNetwork />

<ExistingNetwork />

<RoadLayerControls name="Estimated traffic volume" style="traffic">
  <SequentialLegend colorScale={traffic.colorScale} limits={traffic.limits} />

  {#if $devMode}
    <label>
      <input type="checkbox" bind:checked={$debugOriginalData} />
      Show original data
    </label>
  {/if}
</RoadLayerControls>

<RoadLayerControls name="Gradient" style="gradient">
  <SequentialLegend colorScale={gradient.colorScale} limits={gradient.limits} />
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
