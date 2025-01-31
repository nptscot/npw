<script lang="ts">
  import { SequentialLegend } from "svelte-utils";
  import {
    cnTierColors,
    gradient,
    infraTypeColors,
    levelOfServiceColors,
    reachabilityColors,
    speed,
    traffic,
  } from "../../colors";
  import { QualitativeLegend } from "../../common";
  import { devMode, referenceRoadStyle } from "../../stores";
  import CoreNetwork from "./CoreNetwork.svelte";
  import ExistingNetwork from "./ExistingNetwork.svelte";
  import { debugOriginalData } from "./stores";
</script>

<label>
  Reference layer roads:
  <select bind:value={$referenceRoadStyle}>
    <option value="off">Don't show</option>
    <option value="cn">Core network</option>
    <option value="existing_infra">Existing infrastructure type</option>
    <option value="traffic">Estimated traffic volume</option>
    <option value="gradient">Gradient</option>
    <option value="speed">Estimated speed limit</option>
    <option value="los">Level of Service</option>
    <option value="reachability">Reachability</option>
    <option value="disconnections">Network disconnections</option>
    <option value="precalculated_rnet">Route network (precalculated)</option>
    <option value="calculated_rnet">Route network (calculated)</option>
  </select>
</label>

<details open>
  <summary>Legend</summary>
  {#if $referenceRoadStyle == "cn"}
    <QualitativeLegend colors={cnTierColors} horiz />
    <CoreNetwork />
  {:else if $referenceRoadStyle == "existing_infra"}
    <QualitativeLegend colors={infraTypeColors} />
    <ExistingNetwork />
  {:else if $referenceRoadStyle == "traffic"}
    <SequentialLegend colorScale={traffic.colorScale} limits={traffic.limits} />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $referenceRoadStyle == "gradient"}
    <SequentialLegend
      colorScale={gradient.colorScale}
      limits={gradient.limits}
    />
  {:else if $referenceRoadStyle == "speed"}
    <SequentialLegend colorScale={speed.colorScale} limits={speed.limits} />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $referenceRoadStyle == "los"}
    <QualitativeLegend colors={levelOfServiceColors} horiz />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $referenceRoadStyle == "reachability"}
    <QualitativeLegend colors={reachabilityColors} horiz />
  {:else if $referenceRoadStyle == "disconnections"}
    TODO
  {:else if $referenceRoadStyle == "precalculated_rnet"}
    TODO
  {:else if $referenceRoadStyle == "calculated_rnet"}
    TODO
  {/if}
</details>
