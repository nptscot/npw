<script lang="ts">
  import { basemap } from "../stores";

  function swap() {
    $basemap = $basemap == "streets" ? "light" : "streets";
  }
</script>

<div>
  <button type="button" on:click={swap}>
    <i class="fa-solid fa-layer-group"></i> Basemap
  </button>
</div>

<style>
  div {
    position: absolute;
    bottom: 70px;
    left: 10px;
  }
</style>
