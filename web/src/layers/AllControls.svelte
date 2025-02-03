<script lang="ts">
  import { tierColors } from "../colors";
  import { tier } from "../stores";
  import PickReferenceStyle from "./roads/PickReferenceStyle.svelte";
  import { allControls } from "./stores";

  let basic: HTMLDivElement | null = null;
  let primary: HTMLDivElement | null = null;
  let secondary: HTMLDivElement | null = null;
  let localAccess: HTMLDivElement | null = null;
  let longDistance: HTMLDivElement | null = null;

  $: update(basic, $allControls, [
    "NPT full network",
    "Route network (calculated)",
    "Network disconnections",
    // TODO Ideally visually separate these, above are radios
    "SIMD",
    "Population",
  ]);

  $: update(primary, $allControls, ["High cycling flow"]);

  $: update(secondary, $allControls, ["Medium cycling flow", "Town centres"]);

  $: update(localAccess, $allControls, [
    "Above-minimum cycling flow",
    "Schools",
    "GPs and hospitals",
    "Greenspaces",
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

<details
  open
  style:border="2px solid {tierColors.Primary}"
  style:display={$tier == "Primary" ? "block" : "none"}
>
  <summary>Primary Route Reference Layers</summary>
  <div bind:this={primary} />
</details>

<details
  open
  style:border="2px solid {tierColors.Secondary}"
  style:display={$tier == "Secondary" ? "block" : "none"}
>
  <summary>Secondary Route Reference Layers</summary>
  <div bind:this={secondary} />
</details>

<details
  open
  style:border="2px solid {tierColors.LocalAccess}"
  style:display={$tier == "LocalAccess" ? "block" : "none"}
>
  <summary>Local Access Route Reference Layers</summary>
  <div bind:this={localAccess} />
</details>

<details
  open
  style:border="2px solid {tierColors.LongDistance}"
  style:display={$tier == "LongDistance" ? "block" : "none"}
>
  <summary>Long Disance Route Reference Layers</summary>
  <div bind:this={longDistance} />
</details>

<details open style:border="2px solid black">
  <summary>Basic Reference Layers</summary>
  <PickReferenceStyle />
  <div bind:this={basic} />
</details>
