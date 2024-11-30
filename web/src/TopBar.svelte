<script lang="ts">
  import { tierColors } from "./colors";
  import { tier } from "./stores";
</script>

<div>
  <button
    style:background={$tier == "primary" ? tierColors.primaryRoutes : "grey"}
    on:click={() => ($tier = "primary")}
  >
    Primary routes
  </button>

  <button
    style:background={$tier == "secondary"
      ? tierColors.secondaryRoutes
      : "grey"}
    on:click={() => ($tier = "secondary")}
  >
    Secondary routes
  </button>

  <button
    style:background={$tier == "local access"
      ? tierColors.localAccessRoutes
      : "grey"}
    on:click={() => ($tier = "local access")}
  >
    Local access routes
  </button>

  <button
    style:background={$tier == "long distance"
      ? tierColors.longDistanceRoutes
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
