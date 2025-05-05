<script lang="ts">
  import { Checkbox } from "../common";
  import { currentStage } from "../stores";
  import {
    arterialRoadCoverage,
    cyclingDemandHigh,
    cyclingDemandMedium,
    settlements,
    showUncoveredDemand,
    townCentres,
    uncoveredPopulation,
  } from "./stores";
</script>

{#if $currentStage == "Primary"}
  <h3>Reference layers</h3>

  <Checkbox small bind:checked={$cyclingDemandHigh}>
    High cycling demand corridors
  </Checkbox>
  {#if $cyclingDemandHigh}
    <div style:margin-left="20px">
      <Checkbox small bind:checked={$showUncoveredDemand}>
        Show all demand, even if covered
      </Checkbox>
    </div>
  {/if}

  <Checkbox small bind:checked={$arterialRoadCoverage}>
    Arterial road network coverage
  </Checkbox>
  {#if $arterialRoadCoverage}
    <div style:margin-left="20px">
      <Checkbox small bind:checked={$showUncoveredDemand}>
        Show all arterial roads, even if covered
      </Checkbox>
    </div>
  {/if}
{/if}

{#if $currentStage == "Secondary"}
  <h3>Reference layers</h3>

  <Checkbox small bind:checked={$cyclingDemandMedium}>
    Medium cycling demand corridors
  </Checkbox>
  {#if $cyclingDemandMedium}
    <div style:margin-left="20px">
      <Checkbox small bind:checked={$showUncoveredDemand}>
        Show all demand, even if covered
      </Checkbox>
    </div>
  {/if}

  <Checkbox small bind:checked={$uncoveredPopulation}>
    Unconnected neighbourhoods
  </Checkbox>

  <Checkbox small bind:checked={$townCentres}>Town centres</Checkbox>
{/if}

{#if $currentStage == "LongDistance"}
  <h3>Reference layers</h3>

  <Checkbox small bind:checked={$settlements}>Settlement boundaries</Checkbox>
{/if}
