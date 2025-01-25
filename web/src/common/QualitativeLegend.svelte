<script lang="ts">
  export let colors: { [name: string]: string };
  export let horiz = false;
</script>

{#if horiz}
  <div style="display: flex">
    {#each Object.entries(colors) as [_, color]}
      <span style="background: {color}; width: 100%; border: 1px solid black;">
        &nbsp;
      </span>
    {/each}
  </div>
  <div style="display: flex; justify-content: space-between;">
    {#each Object.entries(colors) as [label, _]}
      <span>{label}</span>
    {/each}
  </div>
{:else}
  <ul>
    {#each Object.entries(colors) as [label, color]}
      <li>
        <span style:background={color} class="vertical-block" />
        {label}
      </li>
    {/each}
  </ul>
{/if}

<style>
  .vertical-block {
    display: block;
    float: left;
    height: 16px;
    width: 30px;
    margin-right: 5px;
    border: 1px solid #999;
  }
</style>
