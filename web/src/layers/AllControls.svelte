<script lang="ts">
  import { allControls } from "./stores";
  import { tier } from "../stores";
  import { tierColors } from "../colors";

  let basic: HTMLDivElement | null = null;
  let primary: HTMLDivElement | null = null;
  let secondary: HTMLDivElement | null = null;
  let localAccess: HTMLDivElement | null = null;
  let longDistance: HTMLDivElement | null = null;

  $: update(basic, $allControls, [
    "route network",
    "simd",
    "population density",
    "core network",
    "existing network",
    "level of service",
    "traffic volume",
    "gradient",
    "reachable network",
  ]);

  $: update(primary, $allControls, ["high npt route coverage"]);

  $: update(secondary, $allControls, ["town centres"]);

  $: update(localAccess, $allControls, ["schools", "gp hospitals"]);

  $: update(longDistance, $allControls, ["urban areas"]);

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

<details open style:border="2px solid black">
  <summary>Basic Reference Layers</summary>
  <div bind:this={basic} />
</details>

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
