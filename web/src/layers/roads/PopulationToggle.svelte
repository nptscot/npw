<script lang="ts">
  import { populationStyle } from "../stores";

  export let name: string;
  export let style: "off" | "population" | "deprived";
  export let icon: string;
  export let relevant = false;

  function toggle() {
    if ($populationStyle == style) {
      $populationStyle = "off";
    } else {
      $populationStyle = style;
    }
  }
</script>

<li>
  <button
    class:selected={$populationStyle == style}
    class:relevant
    on:click={toggle}
  >
    <i class="fa-solid {icon}"></i>
    {name}
  </button>
</li>
