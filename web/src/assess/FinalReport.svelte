<script lang="ts">
  import { BackLink } from "../common";
  import SummarizeStatsWrapper from "../stats/SummarizeStatsWrapper.svelte";
  import { subpage } from "./index";
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network metrics</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<p>Here you can understand the overall performance of your network.</p>

<SummarizeStatsWrapper />
