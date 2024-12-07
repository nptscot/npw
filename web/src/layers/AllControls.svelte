<script lang="ts">
  import { allControls } from "./stores";
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

<details open style:border="2px solid {tierColors.primaryRoutes}">
  <summary>Primary Route Reference Layers</summary>
  <div bind:this={primary} />
</details>

<details open style:border="2px solid {tierColors.secondaryRoutes}">
  <summary>Secondary Route Reference Layers</summary>
  <div bind:this={secondary} />
</details>

<details open style:border="2px solid {tierColors.localAccessRoutes}">
  <summary>Local Access Route Reference Layers</summary>
  <div bind:this={localAccess} />
</details>

<details open style:border="2px solid {tierColors.longDistanceRoutes}">
  <summary>Long Disance Route Reference Layers</summary>
  <div bind:this={longDistance} />
</details>
