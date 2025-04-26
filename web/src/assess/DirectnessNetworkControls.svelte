<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { BackLink, Radio } from "../common";
  import { townCentres } from "../layers/stores";
  import { editsRoadStyle } from "../stores";
  import { showDirectness } from "./";
  import { subpage } from "./index";

  let originalEditsStyle = $editsRoadStyle;

  onMount(() => {
    $townCentres = true;
    $editsRoadStyle = "off";
  });
  onDestroy(() => {
    $townCentres = false;
    $editsRoadStyle = originalEditsStyle;
  });
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Directness network</h2>
</header>

<BackLink on:click={() => ($subpage = "overview")}>
  Back to network assessment
</BackLink>

<p>
  Directness is calculated by checking rhte route between every town centre
  within 5km. You want the quiet routes (which follow high Level of Service
  roads) to match the direct routes. You also want these routes to have high
  Level of Service.
</p>

<Radio
  bind:value={$showDirectness}
  options={[
    ["quiet", "Quiet routes"],
    ["direct", "Direct routes"],
  ]}
  legend="Show routes between town centres"
/>
