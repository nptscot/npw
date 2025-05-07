<script lang="ts">
  import SummarizeStatsWrapper from "../stats/SummarizeStatsWrapper.svelte";
</script>

<div class="wrapper">
  <header class="ds_page-header">
    <h2 class="ds_page-header__title">
      Network Planning Workspace final report
    </h2>
  </header>

  <SummarizeStatsWrapper />
</div>

<style>
  @media screen {
    .wrapper {
      width: 760px;
      margin-left: auto;
      margin-right: auto;
    }
  }
</style>
