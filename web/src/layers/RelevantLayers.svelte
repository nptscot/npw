<script lang="ts">
  import { currentStage } from "../stores";
  import { allControls } from "./stores";

  // TODO This way of doing things is on its way out

  let primary: HTMLDivElement | null = null;
  let secondary: HTMLDivElement | null = null;
  let longDistance: HTMLDivElement | null = null;

  $: update(primary, $allControls, [
    "High cycling demand",
    "Main road coverage",
  ]);

  $: update(secondary, $allControls, ["Medium cycling demand", "Town centres"]);

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

{#if $currentStage == "Primary"}
  <h3>Relevant layers</h3>
  <div bind:this={primary} />
{/if}

{#if $currentStage == "Secondary"}
  <h3>Relevant layers</h3>
  <div bind:this={secondary} />
{/if}

{#if $currentStage == "LongDistance"}
  <h3>Relevant layers</h3>
  <div bind:this={longDistance} />
{/if}
