<script lang="ts">
  import { notNull } from "svelte-utils";
  import { Modal, percent as percent2 } from "../common";
  import { backend, stats } from "../stores";

  let show = false;

  function percent(pct: number): string {
    return `${Math.round(pct * 100)}%`;
  }
</script>

<div><button on:click={() => (show = true)}>Final report</button></div>

<Modal bind:show>
  {#if $stats}
    {#await notNull($backend).getBaselineStats() then baseline}
      <table>
        <thead>
          <tr>
            <th>Metric</th>
            <th>Baseline score</th>
            <th>Score with proposed network</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <th>High cycling demand coverage</th>
            <td>
              {percent2(
                baseline.covered_high_demand,
                baseline.total_high_demand,
              )}
            </td>
            <td>
              {percent2($stats.covered_high_demand, $stats.total_high_demand)}
            </td>
          </tr>
          <tr>
            <th>Medium cycling demand coverage</th>
            <td>
              {percent2(
                baseline.covered_medium_demand,
                baseline.total_medium_demand,
              )}
            </td>
            <td>
              {percent2(
                $stats.covered_medium_demand,
                $stats.total_medium_demand,
              )}
            </td>
          </tr>

          <tr>
            <th>Town centres</th>
            <td>{percent(baseline.percent_reachable_town_centres)}</td>
            <td>{percent($stats.percent_reachable_town_centres)}</td>
          </tr>
          <tr>
            <th>Schools</th>
            <td>{percent(baseline.percent_reachable_schools)}</td>
            <td>{percent($stats.percent_reachable_schools)}</td>
          </tr>
          <tr>
            <th>GPs and hosptials</th>
            <td>{percent(baseline.percent_reachable_gp_hospitals)}</td>
            <td>{percent($stats.percent_reachable_gp_hospitals)}</td>
          </tr>
          <tr>
            <th>Greenspaces</th>
            <td>{percent(baseline.percent_reachable_greenspaces)}</td>
            <td>{percent($stats.percent_reachable_greenspaces)}</td>
          </tr>
          <tr>
            <th>Deprived population coverage</th>
            <td>{percent(baseline.percent_reachable_imd_population)}</td>
            <td>{percent($stats.percent_reachable_imd_population)}</td>
          </tr>
          <tr>
            <th>Population coverage</th>
            <td>{percent(baseline.percent_reachable_population)}</td>
            <td>{percent($stats.percent_reachable_population)}</td>
          </tr>
          <tr>
            <th>Settlements</th>
            <td>{percent(baseline.percent_reachable_settlements)}</td>
            <td>{percent($stats.percent_reachable_settlements)}</td>
          </tr>
        </tbody>
      </table>

      <button on:click={() => (show = false)}>OK</button>
    {/await}
  {/if}
</Modal>
