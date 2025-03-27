<script lang="ts">
  import { currentStage } from "../stores";
  import { allControls } from "./stores";

  let primary: HTMLDivElement | null = null;
  let secondary: HTMLDivElement | null = null;
  let localAccess: HTMLDivElement | null = null;
  let longDistance: HTMLDivElement | null = null;

  $: update(primary, $allControls, [
    "High cycling demand",
    "Main road coverage",
  ]);

  $: update(secondary, $allControls, ["Medium cycling demand", "Town centres"]);

  $: update(localAccess, $allControls, [
    "POIs",
    "Population",
    "Deprived population (SIMD)",
  ]);

  $: update(longDistance, $allControls, ["Settlements"]);

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

<h3>Relevant layers</h3>

{#if $currentStage == "Primary"}
  <div bind:this={primary} />
{/if}

{#if $currentStage == "Secondary"}
  <div bind:this={secondary} />
{/if}

{#if $currentStage == "LocalAccess"}
  <div bind:this={localAccess} />
{/if}

{#if $currentStage == "LongDistance"}
  <div bind:this={longDistance} />
{/if}
