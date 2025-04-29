<script lang="ts">
  import { showReportProblem } from "../stores";

  // There are two components for this, so the modal can exist outside the map
  // and use SG styles.
</script>

<div>
  <button type="button" on:click={() => ($showReportProblem = true)}>
    <i class="fa-solid fa-triangle-exclamation"></i>
    Report problem
  </button>
</div>

<style>
  div {
    position: absolute;
    bottom: 80px;
    left: 10px;
  }
</style>
