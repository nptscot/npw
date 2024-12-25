<script lang="ts">
  import { tierColors } from "../colors";
  import { colorRoutesBy, infraTypes } from "../stores";
</script>

<div>
  <details>
    <summary>Legend</summary>
    {#if $colorRoutesBy == "infra_type"}
      {#each infraTypes as [_, label, color]}
        <span style:color>{label}</span>
        <br />
      {/each}
    {:else}
      {#each Object.entries(tierColors) as [label, color]}
        <span style:color>{label}</span>
        <br />
      {/each}
    {/if}
  </details>
</div>

<style>
  div {
    position: absolute;
    right: 10px;
    bottom: 40px;

    background: white;
    padding: 2px;
  }
</style>
