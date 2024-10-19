<script lang="ts">
  import { coreNetwork } from "./stores";
</script>

<label>
  <input type="checkbox" bind:checked={$coreNetwork} />
  Core network
</label>
