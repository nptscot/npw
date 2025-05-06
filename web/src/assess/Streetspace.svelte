<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { BackLink, percent, prettyPrintDistance } from "../common";
  import { editsRoadStyle, stats } from "../stores";
  import { subpage } from "./index";

  onMount(() => {
    $editsRoadStyle = "edits_deliverability";
  });
  onDestroy(() => {
    $editsRoadStyle = "edits_tier";
  });
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Streetspace deliverability</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

{#if $stats}
  {#if $stats.total_undeliverable_length == 0}
    <p>All of your proposed network fits in the available streetspace.</p>
  {:else}
    <p>
      {percent($stats.total_undeliverable_length, $stats.total_network_length)} of
      your {prettyPrintDistance($stats.total_undeliverable_length)} of proposed network
      doesn't fit in the available streetspace. (For assumptions applied, see the
      manual.) Routes that may not fit are highlighted in red. Click on a red segment
      to resolve the issue, following these approaches:
    </p>

    <ol>
      <li>
        Reduce traffic speeds and volumes locally so that a segregated track is
        no longer necessary to achieve high Level of Service. Use the <i>
          Override infrastructure type
        </i>
        tool and switch to the
        <i>Mixed traffic with traffic measures</i>
         infrastructure type to indicate this.
      </li>
      <li>
        Realign the route (by moving the start/middle/end points as required),
        accepting reduced directness.
      </li>
      <li>
        Downgrade the infrastructure type from full segregation to painted lanes
        (accepting a lower safety Level of Service).
      </li>
    </ol>
  {/if}
{/if}
