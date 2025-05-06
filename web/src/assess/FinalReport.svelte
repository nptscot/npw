<script lang="ts">
  import { Modal } from "../common";
  import SummarizeStatsWrapper from "../stats/SummarizeStatsWrapper.svelte";
  import { subpage } from "./index";

  let show = true;

  $: if (!show) {
    $subpage = "overview";
  }
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network metrics</h2>
</header>

<Modal bind:show>
  <SummarizeStatsWrapper />
</Modal>
