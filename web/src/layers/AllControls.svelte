<script lang="ts">
  import { allControls } from "./stores";
  import { colors } from "../colors";

  let basic: HTMLDivElement | null = null;
  let primary: HTMLDivElement | null = null;
  let secondary: HTMLDivElement | null = null;
  let localAccess: HTMLDivElement | null = null;
  let longDistance: HTMLDivElement | null = null;

  $: update(basic, $allControls, [
    "route network",
    "simd",
    "core network",
    "existing network",
    "level of service",
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

<details open>
  <summary>Basic Reference Layers</summary>
  <div bind:this={basic} />
</details>

<details open style:border="2px solid {colors.primaryRoutes}">
  <summary>Primary Route Reference Layers</summary>
  <div bind:this={primary} />
</details>

<details open style:border="2px solid {colors.secondaryRoutes}">
  <summary>Secondary Route Reference Layers</summary>
  <div bind:this={secondary} />
</details>

<details open style:border="2px solid {colors.localAccessRoutes}">
  <summary>Local Access Route Reference Layers</summary>
  <div bind:this={localAccess} />
</details>

<details open style:border="2px solid {colors.longDistanceRoutes}">
  <summary>Long Disance Route Reference Layers</summary>
  <div bind:this={longDistance} />
</details>
