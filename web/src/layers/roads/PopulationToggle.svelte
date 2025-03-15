<script lang="ts">
  import { populationStyle } from "../stores";

  export let name: string;
  export let style: "off" | "population" | "deprived";

  function toggle() {
    if ($populationStyle == style) {
      $populationStyle = "off";
    } else {
      $populationStyle = style;
    }
  }
</script>

<button
  style:background={$populationStyle == style ? "green" : "grey"}
  style:font-size="15px"
  on:click={toggle}
>
  {name}
</button>
