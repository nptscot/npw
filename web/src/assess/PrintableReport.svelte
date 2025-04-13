<script lang="ts">
  import SummarizeStatsWrapper from "../stats/SummarizeStatsWrapper.svelte";

  // TODO Having trouble getting SG css working on the new page; probably need to construct a <head> section properly
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network Planning Workspace final report</h2>
</header>

<h2>Network metrics</h2>
<SummarizeStatsWrapper />
