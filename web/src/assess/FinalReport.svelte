<script lang="ts">
  import { notNull } from "svelte-utils";
  import { tierColors } from "../colors";
  import { percent as percent2 } from "../common";
  import { backend, stats } from "../stores";
  import { changePage } from "./index";

  function percent(pct: number): string {
    return `${Math.round(pct * 100)}%`;
  }
</script>

<header class="ds_page-header">
  <h2 class="ds_page-header__title">Network metrics</h2>
</header>

<div>
  <button type="button" class="ds_link" on:click={() => changePage("overview")}>
    <i class="fa-solid fa-chevron-left"></i>
    Back to network assessment
  </button>
</div>

<p>The network you have designed performs as follows:</p>

{#if $stats}
  {#await notNull($backend).getBaselineStats() then baseline}
    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Metric</th>
          <th scope="col">Baseline score</th>
          <th scope="col">Score with proposed network</th>
        </tr>
      </thead>
      <tbody>
        <!-- Network -->

        <tr>
          <th scope="row">Safety</th>
          <td>
            {percent2(
              baseline.total_high_los_length,
              baseline.total_network_length,
            )}
          </td>
          <td>
            {percent2(
              $stats.total_high_los_length,
              $stats.total_network_length,
            )}
          </td>
        </tr>

        <tr>
          <th scope="row">Directness</th>
          <td>TODO</td>
          <td>TODO</td>
        </tr>

        <!-- TODO Don't repeat coherence (main road coverage), because it's in primary? -->
        <tr>
          <th scope="row">Coherence (density)</th>
          <td>
            {#if baseline.density_network_in_settlements}
              {Math.round(baseline.density_network_in_settlements)}m
            {:else}
              no routes
            {/if}
          </td>
          <td>
            {#if $stats.density_network_in_settlements}
              {Math.round($stats.density_network_in_settlements)}m
            {:else}
              no routes
            {/if}
          </td>
        </tr>

        <tr>
          <th scope="row">Comfort</th>
          <td>
            {percent2(
              baseline.total_low_gradient_length,
              baseline.total_network_length,
            )}
          </td>
          <td>
            {percent2(
              $stats.total_low_gradient_length,
              $stats.total_network_length,
            )}
          </td>
        </tr>

        <tr>
          <th scope="row">Attractiveness</th>
          <td>TODO</td>
          <td>TODO</td>
        </tr>

        <!-- Primary -->

        <tr>
          <th scope="row" style:background={tierColors.Primary}>
            High cycling demand coverage
          </th>
          <td>
            {percent2(baseline.covered_high_demand, baseline.total_high_demand)}
          </td>
          <td>
            {percent2($stats.covered_high_demand, $stats.total_high_demand)}
          </td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.Primary}>
            Main road coverage
          </th>
          <td>
            {percent2(
              baseline.covered_main_road_length,
              baseline.total_main_road_length,
            )}
          </td>
          <td>
            {percent2(
              $stats.covered_main_road_length,
              $stats.total_main_road_length,
            )}
          </td>
        </tr>

        <!-- Secondary -->

        <tr>
          <th scope="row" style:background={tierColors.Secondary}>
            Medium cycling demand coverage
          </th>
          <td>
            {percent2(
              baseline.covered_medium_demand,
              baseline.total_medium_demand,
            )}
          </td>
          <td>
            {percent2($stats.covered_medium_demand, $stats.total_medium_demand)}
          </td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.Secondary}>
            Town centres
          </th>
          <td>{percent(baseline.percent_reachable_town_centres)}</td>
          <td>{percent($stats.percent_reachable_town_centres)}</td>
        </tr>

        <!-- Local access -->

        <tr>
          <th scope="row" style:background={tierColors.LocalAccess}>Schools</th>
          <td>{percent(baseline.percent_reachable_schools)}</td>
          <td>{percent($stats.percent_reachable_schools)}</td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.LocalAccess}>
            GPs and hosptials
          </th>
          <td>{percent(baseline.percent_reachable_gp_hospitals)}</td>
          <td>{percent($stats.percent_reachable_gp_hospitals)}</td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.LocalAccess}>
            Greenspaces
          </th>
          <td>{percent(baseline.percent_reachable_greenspaces)}</td>
          <td>{percent($stats.percent_reachable_greenspaces)}</td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.LocalAccess}>
            Deprived population coverage
          </th>
          <td>{percent(baseline.percent_reachable_imd_population)}</td>
          <td>{percent($stats.percent_reachable_imd_population)}</td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.LocalAccess}>
            Population coverage
          </th>
          <td>{percent(baseline.percent_reachable_population)}</td>
          <td>{percent($stats.percent_reachable_population)}</td>
        </tr>

        <!-- Long distance -->

        <tr>
          <th scope="row" style:background={tierColors.LongDistance}>
            Settlements
          </th>
          <td>{percent(baseline.percent_reachable_settlements)}</td>
          <td>{percent($stats.percent_reachable_settlements)}</td>
        </tr>
      </tbody>
    </table>
  {/await}
{/if}
