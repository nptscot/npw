<script lang="ts">
  import { colors } from "./colors";
  import { tier } from "./stores";
</script>

<div>
  <button
    style:background={$tier == "primary" ? colors.primaryRoutes : "grey"}
    on:click={() => ($tier = "primary")}
  >
    Primary routes
  </button>

  <button
    style:background={$tier == "secondary" ? colors.secondaryRoutes : "grey"}
    on:click={() => ($tier = "secondary")}
  >
    Secondary routes
  </button>

  <button
    style:background={$tier == "local access"
      ? colors.localAccessRoutes
      : "grey"}
    on:click={() => ($tier = "local access")}
  >
    Local access routes
  </button>

  <button
    style:background={$tier == "long distance"
      ? colors.longDistanceRoutes
      : "grey"}
    on:click={() => ($tier = "long distance")}
  >
    Long distance routes
  </button>
</div>

<style>
  div {
    display: flex;
  }

  button {
    flex: 1;
  }
</style>
