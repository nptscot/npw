<script lang="ts">
  import { Checkbox } from "../common";
  import { currentStage, devMode } from "../stores";
  import {
    cyclingDemandHigh,
    cyclingDemandMedium,
    debugAllCyclingDemand,
    debugCyclingDemandMin,
    mainRoadCoverage,
    settlements,
    showUncovered,
    townCentres,
  } from "./stores";
</script>

{#if $currentStage == "Primary"}
  <h3>Relevant layers</h3>

  <Checkbox small bind:checked={$cyclingDemandHigh}>
    High cycling demand
  </Checkbox>
  {#if $cyclingDemandHigh}
    <div style:margin-left="20px">
      <Checkbox small bind:checked={$showUncovered}>
        Show all demand, even if covered
      </Checkbox>

      {#if $devMode}
        <Checkbox small bind:checked={$debugAllCyclingDemand}>
          Debug all demand
        </Checkbox>

        {#if $debugAllCyclingDemand}
          <label>
            Show demand above:
            <input type="number" bind:value={$debugCyclingDemandMin} />
          </label>
        {/if}
      {/if}
    </div>
  {/if}

  <Checkbox small bind:checked={$mainRoadCoverage}>Main road coverage</Checkbox>
  {#if $mainRoadCoverage}
    <div style:margin-left="20px">
      <Checkbox small bind:checked={$showUncovered}>
        Show all demand, even if covered
      </Checkbox>
    </div>
  {/if}
{/if}

{#if $currentStage == "Secondary"}
  <h3>Relevant layers</h3>

  <Checkbox small bind:checked={$cyclingDemandMedium}>
    Medium cycling demand
  </Checkbox>
  {#if $cyclingDemandMedium}
    <div style:margin-left="20px">
      <Checkbox small bind:checked={$showUncovered}>
        Show all demand, even if covered
      </Checkbox>

      {#if $devMode}
        <Checkbox small bind:checked={$debugAllCyclingDemand}>
          Debug all demand
        </Checkbox>

        {#if $debugAllCyclingDemand}
          <label>
            Show demand above:
            <input type="number" bind:value={$debugCyclingDemandMin} />
          </label>
        {/if}
      {/if}
    </div>
  {/if}

  <Checkbox small bind:checked={$townCentres}>Town centres</Checkbox>
{/if}

{#if $currentStage == "LongDistance"}
  <h3>Relevant layers</h3>

  <Checkbox small bind:checked={$settlements}>Settlements</Checkbox>
{/if}
