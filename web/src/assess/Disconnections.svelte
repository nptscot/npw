<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { componentColors } from "../colors";
  import { BackLink, prettyPrintDistance } from "../common";
  import { backgroundLayer, connectedComponents, map } from "../stores";
  import { subpage } from "./index";

  onMount(() => {
    $backgroundLayer = "disconnections";
  });
  onDestroy(() => {
    $backgroundLayer = "off";
  });

  let n = 15;
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network splits</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<p>
  The network you create should usually all be connected as one piece per
  settlement. If there are gaps between pieces, then return to the appropriate
  tier and fill in the gaps. If it looks like some routes should be connected
  but the map shows otherwise, then try zooming in and looking for very small
  road segments between dual carriageways or in complex junctions to fix the
  connection.
</p>

<p>Click a piece to see it:</p>
<ul>
  {#each $connectedComponents.component_lengths.slice(0, n) as length, idx}
    <li>
      <!-- svelte-ignore a11y-invalid-attribute -->
      <a
        style:color={componentColors[idx % componentColors.length]}
        href="#"
        on:click|preventDefault={() =>
          $map?.fitBounds($connectedComponents.component_bboxes[idx])}
      >
        {prettyPrintDistance(length)}
      </a>
    </li>
  {/each}
  {#if $connectedComponents.component_lengths.length > n}
    <p>
      ({$connectedComponents.component_lengths.length} components total; only {n}
      largest shown)
    </p>
  {/if}
</ul>
