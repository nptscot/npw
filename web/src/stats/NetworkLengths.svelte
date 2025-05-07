<script lang="ts">
  import { prettyPrintDistance } from "../common";
  import { backend } from "../stores";
</script>

{#if $backend}
  {#await $backend.getNetworkLengths() then data}
    <h4>Length by infrastructure type</h4>
    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Infrastructure type</th>
          <th scope="col">Network length</th>
        </tr>
      </thead>
      <tbody>
        {#each Object.entries(data.infra_type) as [key, value]}
          <tr>
            <th scope="row">{key}</th>
            <td>{prettyPrintDistance(value)}</td>
          </tr>
        {/each}
      </tbody>
    </table>

    <h4>Length by Level of Service</h4>
    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Level of Service</th>
          <th scope="col">Network length</th>
        </tr>
      </thead>
      <tbody>
        {#each Object.entries(data.los) as [key, value]}
          <tr>
            <th scope="row">{key}</th>
            <td>{prettyPrintDistance(value)}</td>
          </tr>
        {/each}
      </tbody>
    </table>

    <h4>Length by tier</h4>
    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Tier</th>
          <th scope="col">Network length</th>
        </tr>
      </thead>
      <tbody>
        {#each Object.entries(data.tier) as [key, value]}
          <tr>
            <th scope="row">{key}</th>
            <td>{prettyPrintDistance(value)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/await}
{/if}
