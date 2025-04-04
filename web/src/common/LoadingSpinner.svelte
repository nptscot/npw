<script lang="ts">
  import { loadingSpinners } from "../stores";
</script>

<div>{$loadingSpinners}</div>

<style>
  div {
    position: absolute;
    top: 80px;
    left: 10px;
    background-color: white;
    padding: 4px;
  }
</style>
