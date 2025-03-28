<script lang="ts">
  import { changePage } from "./index";
  import ReportContents from "./ReportContents.svelte";
  import PrintableReport from "./ReportContents.svelte";

  let reportContent: HTMLDivElement | undefined = undefined;

  function makeReport() {
    let newTab = window.open("", "_blank");
    if (!newTab || !reportContent) {
      return;
    }

    reportContent.style.display = "block";
    newTab.document.write(reportContent.outerHTML);
    reportContent.style.display = "none";
  }
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network metrics</h2>
</header>

<div>
  <button type="button" class="ds_link" on:click={() => changePage("overview")}>
    <i class="fa-solid fa-chevron-left"></i>
    Back to network assessment
  </button>
</div>

<button class="ds_button" on:click={makeReport}>Print report</button>

<ReportContents />

<!-- Render an invisible component to steal its HTML -->
<div bind:this={reportContent} style:display="none">
  <PrintableReport />
</div>
