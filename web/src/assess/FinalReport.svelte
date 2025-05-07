<script lang="ts">
  import { BackLink, Modal } from "../common";
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

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<Modal bind:show>
  <SummarizeStatsWrapper />
</Modal>
