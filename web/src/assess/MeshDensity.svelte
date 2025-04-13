<script lang="ts">
  import { onDestroy, onMount } from "svelte";
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

<p>
  This map measures the mesh density of your network. Only areas touching a
  settlement are shown. For any squares with poor coverage, you can return to an
  appropriate tier and add more routes.
</p>
