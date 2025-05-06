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
  <h2 class="ds_page-header__title">Mesh density</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<p>
  This map measures the mesh density of your network - a key feature of a good
  quality, coherent network. Mesh density has been calculated based on the
  spacing of primary and secondary routes, within settlements only. Mesh density
  in dense urban centres should be high (green), while in lower density suburban
  areas a lower mesh density may be acceptable (red). For any squares with an
  unacceptably low mesh density, return to the primary secondary tier and add
  more routes.
</p>
