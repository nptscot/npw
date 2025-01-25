<script lang="ts">
  import { SequentialLegend } from "svelte-utils";
  import {
    gradient,
    infraTypeColors,
    levelOfServiceColors,
    reachabilityColors,
    speed,
    tierColors,
    traffic,
  } from "../../colors";
  import { QualitativeLegend } from "../../common";
  import { devMode, roadStyle } from "../../stores";
  import CoreNetwork from "./CoreNetwork.svelte";
  import ExistingNetwork from "./ExistingNetwork.svelte";
  import { debugOriginalData } from "./stores";
</script>

<label>
  Show roads:
  <select bind:value={$roadStyle}>
    <option value="off">Don't show</option>
    <option value="current_infra">Current infrastructure type</option>
    <option value="current_tier">Current tier</option>
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
  {#if $roadStyle == "current_infra"}
    <QualitativeLegend colors={infraTypeColors} />
  {:else if $roadStyle == "current_tier"}
    <QualitativeLegend colors={tierColors} horiz />
  {:else if $roadStyle == "cn"}
    <QualitativeLegend colors={tierColors} horiz />
    <CoreNetwork />
  {:else if $roadStyle == "existing_infra"}
    <QualitativeLegend colors={infraTypeColors} />
    <ExistingNetwork />
  {:else if $roadStyle == "traffic"}
    <SequentialLegend colorScale={traffic.colorScale} limits={traffic.limits} />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $roadStyle == "gradient"}
    <SequentialLegend
      colorScale={gradient.colorScale}
      limits={gradient.limits}
    />
  {:else if $roadStyle == "speed"}
    <SequentialLegend colorScale={speed.colorScale} limits={speed.limits} />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $roadStyle == "los"}
    <QualitativeLegend colors={levelOfServiceColors} horiz />

    {#if $devMode}
      <label>
        <input type="checkbox" bind:checked={$debugOriginalData} />
        Show original data
      </label>
    {/if}
  {:else if $roadStyle == "reachability"}
    <QualitativeLegend colors={reachabilityColors} horiz />
  {:else if $roadStyle == "disconnections"}
    TODO
  {:else if $roadStyle == "precalculated_rnet"}
    TODO
  {:else if $roadStyle == "calculated_rnet"}
    TODO
  {/if}
</details>
