<script lang="ts">
  import { networkAssessmentColor, tierColors } from "../colors";
  import { currentStage } from "../stores";
  import PickEditsStyle from "./roads/PickEditsStyle.svelte";
  import { allControls } from "./stores";

  let basic: HTMLDivElement | null = null;
  let primary: HTMLDivElement | null = null;
  let secondary: HTMLDivElement | null = null;
  let localAccess: HTMLDivElement | null = null;
  let longDistance: HTMLDivElement | null = null;
  let networkAssessment: HTMLDivElement | null = null;

  $: update(basic, $allControls, ["SIMD", "Population"]);

  $: update(primary, $allControls, ["High cycling flow", "Main road coverage"]);

  $: update(secondary, $allControls, ["Medium cycling flow", "Town centres"]);

  $: update(localAccess, $allControls, ["POIs"]);

  $: update(longDistance, $allControls, ["Urban areas", "Settlements"]);

  $: update(networkAssessment, $allControls, ["Mesh density"]);

  function update(
    container: HTMLDivElement | null,
    allControls: Map<string, HTMLDivElement>,
    order: string[],
  ) {
    if (container) {
      container.innerHTML = "";
      for (let name of order) {
        let obj = allControls.get(name);
        if (obj) {
          container.appendChild(obj);
        }
      }
    }
  }
</script>

<PickEditsStyle />

{#if $currentStage == "Primary"}
  <details open style:border="2px solid {tierColors.Primary}">
    <summary>Primary Route Reference Layers</summary>
    <div bind:this={primary} />
  </details>
{/if}

{#if $currentStage == "Secondary"}
  <details open style:border="2px solid {tierColors.Secondary}">
    <summary>Secondary Route Reference Layers</summary>
    <div bind:this={secondary} />
  </details>
{/if}

{#if $currentStage == "LocalAccess"}
  <details open style:border="2px solid {tierColors.LocalAccess}">
    <summary>Local Access Route Reference Layers</summary>
    <div bind:this={localAccess} />
  </details>
{/if}

{#if $currentStage == "LongDistance"}
  <details open style:border="2px solid {tierColors.LongDistance}">
    <summary>Long Disance Route Reference Layers</summary>
    <div bind:this={longDistance} />
  </details>
{/if}

{#if $currentStage == "assessment"}
  <details open style:border="2px solid {networkAssessmentColor}">
    <summary>Network assessment Reference Layers</summary>
    <div bind:this={networkAssessment} />
  </details>
{/if}

<details open style:border="2px solid black">
  <summary>Basic Reference Layers</summary>
  <div bind:this={basic} />
</details>
