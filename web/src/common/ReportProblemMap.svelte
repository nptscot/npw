<script lang="ts">
  import { showReportProblem, zoom } from "../stores";

  // There are two components for this, so the modal can exist outside the map
  // and use SG styles.
</script>

<div>
  <button
    type="button"
    on:click={() => ($showReportProblem = true)}
    disabled={$zoom != null && $zoom < 14}
    title={$zoom != null && $zoom < 14
      ? "Please zoom in further to show the problem"
      : undefined}
  >
    <i class="fa-solid fa-triangle-exclamation"></i>
    Report problem
  </button>
</div>

<style>
  div {
    position: absolute;
    bottom: 40px;
    left: 10px;
  }
</style>
