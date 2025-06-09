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

  // Note we don't bind:checked for the Checkboxes here -- when this component
  // gets destroyed, that was changing the checkbox stores to undefined
  // for some reason
</script>

{#if $currentStage == "Primary"}
  <h3>Reference layers</h3>

  <Checkbox
    small
    checked={$cyclingDemandHigh}
    on:change={(e) => cyclingDemandHigh.set(e.detail)}
  >
    High cycling demand corridors
  </Checkbox>
  {#if $cyclingDemandHigh}
    <div style:margin-left="20px">
      <Checkbox
        small
        checked={$showUncoveredDemand}
        on:change={(e) => showUncoveredDemand.set(e.detail)}
      >
        Show all demand, even if covered
      </Checkbox>
    </div>
  {/if}

  <Checkbox
    small
    checked={$arterialRoadCoverage}
    on:change={(e) => arterialRoadCoverage.set(e.detail)}
  >
    Arterial road network coverage
  </Checkbox>
  {#if $arterialRoadCoverage}
    <div style:margin-left="20px">
      <Checkbox
        small
        checked={$showUncoveredDemand}
        on:change={(e) => showUncoveredDemand.set(e.detail)}
      >
        Show all arterial roads, even if covered
      </Checkbox>
    </div>
  {/if}
{/if}

{#if $currentStage == "Secondary"}
  <h3>Reference layers</h3>

  <Checkbox
    small
    checked={$cyclingDemandMedium}
    on:change={(e) => cyclingDemandMedium.set(e.detail)}
  >
    Medium cycling demand corridors
  </Checkbox>
  {#if $cyclingDemandMedium}
    <div style:margin-left="20px">
      <Checkbox
        small
        checked={$showUncoveredDemand}
        on:change={(e) => showUncoveredDemand.set(e.detail)}
      >
        Show all demand, even if covered
      </Checkbox>
    </div>
  {/if}

  <Checkbox
    small
    checked={$uncoveredPopulation}
    on:change={(e) => uncoveredPopulation.set(e.detail)}
  >
    Unconnected neighbourhoods
  </Checkbox>
  {#if $uncoveredPopulation}
    <div style:margin-left="20px">
      <button class="ds_button ds_button--secondary" on:click={gotoUncovered}>
        Jump to an unconnected neighbourhood
      </button>
    </div>
  {/if}

  <Checkbox
    small
    checked={$townCentres}
    on:change={(e) => townCentres.set(e.detail)}
  >
    Town centres
  </Checkbox>
{/if}

{#if $currentStage == "LongDistance"}
  <h3>Reference layers</h3>

  <Checkbox
    small
    checked={$settlements}
    on:change={(e) => settlements.set(e.detail)}
  >
    Settlement boundaries
  </Checkbox>
{/if}
