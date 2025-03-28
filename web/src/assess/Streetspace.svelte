<script lang="ts">
  import { percent, prettyPrintDistance } from "../common";
  import { stats } from "../stores";
  import { changePage } from "./index";
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Streetspace deliverability</h2>
</header>

<div>
  <button type="button" class="ds_link" on:click={() => changePage("overview")}>
    <i class="fa-solid fa-chevron-left"></i>
    Back to network assessment
  </button>
</div>

{#if $stats}
  {#if $stats.total_undeliverable_length == 0}
    <p>All of your proposed network fits in the available streetspace.</p>
  {:else}
    <p>
      {percent($stats.total_undeliverable_length, $stats.total_network_length)} of
      your {prettyPrintDistance($stats.total_undeliverable_length)} of proposed network
      doesn't fit in the available streetspace. Click on a red segment to fix this,
      following these approaches:
    </p>

    <ol>
      <li>Realign the route</li>
      <li>
        Pick an infrastructure type aside from segregated track, accepting a
        lower Level of Service
      </li>
    </ol>
  {/if}
{/if}
