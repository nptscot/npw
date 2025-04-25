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
      doesn't fit in the available streetspace. Click on a red segment to fix this,
      following these approaches:
    </p>

    <ol>
      <li>
        Reduce traffic speed and volume so that a segregated track is not
        necessary for high Level of Service. Switch to the <i>
          Mixed traffic with traffic measures
        </i>
        infrastructure type to indicate this.
      </li>
      <li>Realign the route</li>
      <li>
        Pick an infrastructure type aside from segregated track, accepting a
        lower Level of Service
      </li>
    </ol>
  {/if}
{/if}
