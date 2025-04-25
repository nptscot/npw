<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { BackLink } from "../common";
  import { gridMeshDensity } from "../layers/stores";
  import { subpage } from "./index";

  onMount(() => {
    $gridMeshDensity = true;
  });
  onDestroy(() => {
    $gridMeshDensity = false;
  });
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network coverage</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<p>
  This map measures the mesh density of your network. Only areas touching a
  settlement are shown. For any squares with poor coverage, you can return to an
  appropriate tier and add more routes.
</p>
