<script lang="ts">
  import { infraTypeColors, tierColors } from "../colors";
  import { roadStyle } from "../stores";
  import { QualitativeLegend } from "./index";
</script>

<label>
  Show roads:
  <select bind:value={$roadStyle}>
    <option value="off">Don't show</option>
    <option value="current_infra">Current infrastructure type</option>
    <option value="current_tier">Tier</option>
    <option value="cn">Core network</option>
    <option value="existing_infra">Existing infrastructure type</option>
    <option value="traffic">Estimated traffic volume</option>
    <option value="gradient">Gradient</option>
    <option value="speed">Estimated speed limit</option>
    <option value="los">Level of Service</option>
    <option value="reachability">Reachability</option>
    <option value="disconnections">Network disconnections</option>
    <option value="precalculated_rnet">Route network (precalculated)</option>
  </select>
</label>

<details>
  <summary>Legend</summary>
  {#if $roadStyle == "current_infra"}
    <QualitativeLegend colors={infraTypeColors} />
  {:else if $roadStyle == "current_tier"}
    <QualitativeLegend colors={tierColors} />
  {/if}
</details>
