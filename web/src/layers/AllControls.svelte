<script lang="ts">
  import { tierColors } from "../colors";
  import { tier } from "../stores";
  import PickEditsStyle from "./roads/PickEditsStyle.svelte";
  import { allControls } from "./stores";

  let basic: HTMLDivElement | null = null;
  let primary: HTMLDivElement | null = null;
  let secondary: HTMLDivElement | null = null;
  let localAccess: HTMLDivElement | null = null;
  let longDistance: HTMLDivElement | null = null;

  $: update(basic, $allControls, ["SIMD", "Population"]);

  $: update(primary, $allControls, ["High cycling flow"]);

  $: update(secondary, $allControls, ["Medium cycling flow", "Town centres"]);

  $: update(localAccess, $allControls, [
    "POIs",
    "Mesh density (grid)",
    "Mesh density (area)",
  ]);

  $: update(longDistance, $allControls, ["Urban areas", "Settlements"]);

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

{#if $tier == "Primary"}
  <details open style:border="2px solid {tierColors.Primary}">
    <summary>Primary Route Reference Layers</summary>
    <div bind:this={primary} />
  </details>
{/if}

{#if $tier == "Secondary"}
  <details open style:border="2px solid {tierColors.Secondary}">
    <summary>Secondary Route Reference Layers</summary>
    <div bind:this={secondary} />
  </details>
{/if}

{#if $tier == "LocalAccess"}
  <details open style:border="2px solid {tierColors.LocalAccess}">
    <summary>Local Access Route Reference Layers</summary>
    <div bind:this={localAccess} />
  </details>
{/if}

{#if $tier == "LongDistance"}
  <details open style:border="2px solid {tierColors.LongDistance}">
    <summary>Long Disance Route Reference Layers</summary>
    <div bind:this={longDistance} />
  </details>
{/if}

<details open style:border="2px solid black">
  <summary>Basic Reference Layers</summary>
  <div bind:this={basic} />
</details>
