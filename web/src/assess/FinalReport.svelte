<script lang="ts">
  import SummarizeStatsWrapper from "../stats/SummarizeStatsWrapper.svelte";
  import { subpage } from "./index";
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network metrics</h2>
</header>

<div>
  <button
    type="button"
    class="ds_link"
    on:click={() => ($subpage = "overview")}
  >
    <i class="fa-solid fa-chevron-left"></i>
    Back to network assessment
  </button>
</div>

<SummarizeStatsWrapper />
