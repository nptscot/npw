<script lang="ts">
  import { notNull } from "svelte-utils";
  import { tierColors } from "../colors";
  import { percent as percent2 } from "../common";
  import {
    backend,
    lastUpdateSlowStats,
    mutationCounter,
    slowStats,
    stats,
  } from "../stores";
  import type { Stats } from "../types";

  function percent(pct: number): string {
    return `${Math.round(pct * 100)}%`;
  }

  function percent3(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }

    return Math.round((x / total) * 100);
  }

  function stepLessThanOrEqual(pct: number, steps: number[]): string {
    if (pct <= steps[0]) {
      return "very poor";
    }
    if (pct <= steps[1]) {
      return "poor";
    }
    if (pct <= steps[2]) {
      return "medium";
    }
    if (pct <= steps[3]) {
      return "good";
    }
    return "very good";
  }

  function stepGreaterThan(pct: number, steps: number[]): string {
    if (pct > steps[0]) {
      return "very poor";
    }
    if (pct > steps[1]) {
      return "poor";
    }
    if (pct > steps[2]) {
      return "medium";
    }
    if (pct > steps[3]) {
      return "good";
    }
    return "very good";
  }

  function safety(s: Stats): string {
    let pct = percent3(
      s.total_high_los_main_roads_length,
      s.total_main_road_length,
    );
    let rating = stepLessThanOrEqual(pct, [20, 40, 60, 80]);
    return `${pct}% (${rating})`;
  }

  function coherentDensity(s: Stats): string {
    if (!s.density_network_in_settlements) {
      return "no routes";
    }
    let rating = stepGreaterThan(
      s.density_network_in_settlements,
      [1000, 500, 400, 250],
    );
    return `${Math.round(s.density_network_in_settlements)}m (${rating})`;
  }

  function comfort(s: Stats): string {
    let pct = percent3(s.total_low_gradient_length, s.total_network_length);
    let rating = stepLessThanOrEqual(pct, [10, 20, 40, 60]);
    return `${pct}% (${rating})`;
  }

  function attractiveness(s: Stats): string {
    let pct = percent3(s.total_attractive_length, s.total_network_length);
    // First threshold will almost never happen; this is a deliberate choice
    let rating = stepLessThanOrEqual(pct, [0, 25, 50, 75]);
    return `${pct}% (${rating})`;
  }

  function directness(s: { average_weighted_directness: number }): string {
    let rating = stepGreaterThan(
      s.average_weighted_directness,
      [1.5, 1.4, 1.3, 1.2],
    );
    return `${s.average_weighted_directness.toFixed(1)}x (${rating})`;
  }
</script>

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
        <tr>
          <th scope="row">Safety</th>
          <td>{safety(baseline)}</td>
          <td>{safety($stats)}</td>
        </tr>

        <tr>
          <th scope="row">Directness</th>
          <td>{directness(baseline)}</td>
          {#if $slowStats && $lastUpdateSlowStats == $mutationCounter}
            <td>{directness($slowStats)}</td>
          {:else}
            <td>Need to recalculate</td>
          {/if}
        </tr>

        <tr>
          <th scope="row">Coherence (density)</th>
          <td>{coherentDensity(baseline)}</td>
          <td>{coherentDensity($stats)}</td>
        </tr>

        <tr>
          <th scope="row">Comfort</th>
          <td>{comfort(baseline)}</td>
          <td>{comfort($stats)}</td>
        </tr>

        <tr>
          <th scope="row">Attractiveness</th>
          <td>{attractiveness(baseline)}</td>
          <td>{attractiveness($stats)}</td>
        </tr>
      </tbody>
    </table>

    <h3>Per-tier metrics</h3>

    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Metric</th>
          <th scope="col">Baseline score</th>
          <th scope="col">Score with proposed network</th>
        </tr>
      </thead>
      <tbody>
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

        <tr>
          <th scope="row" style:background={tierColors.Secondary}>
            Deprived population coverage
          </th>
          <td>{percent(baseline.percent_reachable_imd_population)}</td>
          <td>{percent($stats.percent_reachable_imd_population)}</td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.Secondary}>
            Population coverage
          </th>
          <td>{percent(baseline.percent_reachable_population)}</td>
          <td>{percent($stats.percent_reachable_population)}</td>
        </tr>

        <!-- Local access -->

        <tr>
          <th scope="row" style:background={tierColors.LocalAccess}>Schools</th>
          <td>{percent(baseline.percent_reachable_schools)}</td>
          <td>{percent($stats.percent_reachable_schools)}</td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.LocalAccess}>
            GPs and hospitals
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
