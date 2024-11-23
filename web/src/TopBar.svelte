<script lang="ts">
  import { colors } from "./colors";
</script>

<div>
  <button style:background={colors.primaryRoutes}>Primary routes</button>

  <button style:background={colors.secondaryRoutes}>Secondary routes</button>

  <button style:background={colors.localAccessRoutes}>
    Local access routes
  </button>

  <button style:background={colors.longDistanceRoutes}>
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
