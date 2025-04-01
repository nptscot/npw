<script lang="ts">
  import { changePage } from "./index";
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Population coverage</h2>
</header>

<div>
  <button type="button" class="ds_link" on:click={() => changePage("overview")}>
    <i class="fa-solid fa-chevron-left"></i>
    Back to network assessment
  </button>
</div>

<p>
  Zones with a red outline are not connected by the current network. To connect
  a zone, return to the appropriate tier and draw a route.
</p>
