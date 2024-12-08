<script lang="ts">
  import { infraTypes } from "../stores";
</script>

<div>
  <details>
    <summary>Legend</summary>
    {#each infraTypes as [_, label, color]}
      <span style:color>{label}</span>
      <br />
    {/each}
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
