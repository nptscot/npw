<script lang="ts">
  import { currentStage } from "../stores";
  import { allControls } from "./stores";

  let primary: HTMLDivElement | null = null;
  let secondary: HTMLDivElement | null = null;
  let localAccess: HTMLDivElement | null = null;
  let longDistance: HTMLDivElement | null = null;
  let networkAssessment: HTMLDivElement | null = null;

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

{#if $currentStage == "Primary"}
  <h3>Relevant layers</h3>
  <div bind:this={primary} />
{/if}

{#if $currentStage == "Secondary"}
  <h3>Relevant layers</h3>
  <div bind:this={secondary} />
{/if}

{#if $currentStage == "LocalAccess"}
  <h3>Relevant layers</h3>
  <div bind:this={localAccess} />
{/if}

{#if $currentStage == "LongDistance"}
  <h3>Relevant layers</h3>
  <div bind:this={longDistance} />
{/if}

{#if $currentStage == "assessment"}
  <h3>Relevant layers</h3>
  <div bind:this={networkAssessment} />
{/if}
