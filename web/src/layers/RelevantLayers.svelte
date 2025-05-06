<script lang="ts">
  import { Checkbox } from "../common";
  import { backend, currentStage, map } from "../stores";
  import {
    arterialRoadCoverage,
    cyclingDemandHigh,
    cyclingDemandMedium,
    settlements,
    showUncoveredDemand,
    townCentres,
    uncoveredPopulation,
  } from "./stores";

  async function gotoUncovered() {
    let zones = await $backend!.getDataZones();
    let centroids = zones.features
      .filter((z) => !z.properties.reachable)
      .map((z) => z.properties.centroid);
    if (centroids.length == 0) {
      window.alert("All neighbourhoods are connected already");
      return;
    }
    // We don't need to consistently jump around in any order; literally just pick any random one
    let center = centroids[Math.floor(Math.random() * centroids.length)];
    // TODO Zoom in if we're too far away?
    $map!.easeTo({
      center,
    });
  }
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
  {#if $uncoveredPopulation}
    <div style:margin-left="20px">
      <button class="ds_button ds_button--secondary" on:click={gotoUncovered}>
        Jump to an unconnected neighbourhood
      </button>
    </div>
  {/if}

  <Checkbox small bind:checked={$townCentres}>Town centres</Checkbox>
{/if}

{#if $currentStage == "LongDistance"}
  <h3>Reference layers</h3>

  <Checkbox small bind:checked={$settlements}>Settlement boundaries</Checkbox>
{/if}
