<script lang="ts">
  import { coreNetwork, schools, townCentres, gpHospitals } from "./stores";
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

<label>
  <input type="checkbox" bind:checked={$gpHospitals} />
  GPs and hospitals
</label>
