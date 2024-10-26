<script lang="ts">
  import { allControls } from "./stores";

  // TODO The order gets messed up while updating components

  let container: HTMLDivElement | null = null;

  function update(
    container: HTMLDivElement | null,
    allControls: HTMLDivElement[],
  ) {
    if (container) {
      container.innerHTML = "";
      for (let obj of allControls) {
        container.appendChild(obj);
      }
    }
  }
  $: update(container, $allControls);
</script>

<div bind:this={container} />
