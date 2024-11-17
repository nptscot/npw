<script lang="ts">
  import { notNull } from "svelte-utils";
  import { backend, stats, mode } from "./stores";
  import { onMount } from "svelte";
  import { schools, gpHospitals, townCentres, imdZones } from "./layers/stores";

  async function recalc() {
    $stats = await $backend!.recalculateStats();
  }

  onMount(async () => {
    if ($stats == null) {
      await recalc();
    }
  });

  function percent(x: number): string {
    return `${(100 * x).toFixed(1)}%`;
  }
</script>

<button on:click={recalc}>Recalculate</button>

{#if $stats}
  <p>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a
      href="#"
      on:click|preventDefault={() =>
        ($mode = {
          kind: "debug-worst-routes",
          routes: notNull($stats).worst_directness_routes,
        })}
    >
      Average weighted directness
    </a>
    : {$stats.average_weighted_directness.toFixed(1)}x
  </p>

  <p>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|preventDefault={() => ($schools = true)}>Schools</a>
    :
    <b>{percent($stats.percent_reachable_schools)}</b>
  </p>

  <p>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|preventDefault={() => ($gpHospitals = true)}>
      GPs and hospitals
    </a>
    :
    <b>{percent($stats.percent_reachable_gp_hospitals)}</b>
  </p>

  <p>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|preventDefault={() => ($townCentres = true)}>
      Town Centres
    </a>
    :
    <b>{percent($stats.percent_reachable_town_centres)}</b>
  </p>

  <p>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|preventDefault={() => ($imdZones = true)}>SIMD</a>
    :
    <b>{percent($stats.percent_reachable_imd_population)}</b>
  </p>

  <details>
    <summary>Percent of demand by infrastructure type</summary>
    <ul>
      {#each Object.entries($stats.od_percents_infra_type).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
        <li>{key}: {percent(pct)}</li>
      {/each}
    </ul>
  </details>

  <details>
    <summary>Percent of demand by level of service:</summary>
    <ul>
      {#each Object.entries($stats.od_percents_los).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
        <li>{key}: {percent(pct)}</li>
      {/each}
    </ul>
  </details>
{/if}
