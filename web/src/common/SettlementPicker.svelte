<script lang="ts">
  import { backend, map } from "../stores";

  let value = "";

  function warp() {
    if (!value) {
      return;
    }
    let bounds: [number, number, number, number] = JSON.parse(value);
    $map?.fitBounds(bounds, { padding: 20, duration: 2000 });
  }
</script>

{#if $backend}
  {#await $backend.getSettlementLocations() then list}
    <div>
      <i class="fa-solid fa-city"></i>
      <select bind:value on:change={warp}>
        <option value="">Jump to settlement</option>
        {#each list as [name, bounds]}
          <option value={JSON.stringify(bounds)}>{name}</option>
        {/each}
      </select>
    </div>
  {/await}
{/if}

<style>
  div {
    position: absolute;
    top: 60px;
    left: 50px;

    background: white;
    padding: 4px;
  }
</style>
