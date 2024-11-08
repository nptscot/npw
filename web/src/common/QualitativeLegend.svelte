<script lang="ts">
  // Vertical layout, for qualitative color schemes
  export let colors: { [name: string]: string };
</script>

<ul>
  {#each Object.entries(colors) as [label, color]}
    <li>
      <span style:background={color} />
      {label}
    </li>
  {/each}
</ul>

<style>
  span {
    display: block;
    float: left;
    height: 16px;
    width: 30px;
    margin-right: 5px;
    border: 1px solid #999;
  }
</style>
