<script lang="ts">
  import { coreNetwork, schools, townCentres } from "./stores";
</script>

<label>
  <input type="checkbox" bind:checked={$coreNetwork} />
  Core network
</label>

<label>
  <input type="checkbox" bind:checked={$schools} />
  Schools
</label>

<label>
  <input type="checkbox" bind:checked={$townCentres} />
  Town centres
</label>
