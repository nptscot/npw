<script lang="ts">
  // This component must be protected by SummarizeStatsWrapper, which makes
  // sure $stats and $slowStats are up-to-date
  import {
    Chart as ChartJS,
    Legend,
    LineElement,
    PointElement,
    RadialLinearScale,
    Title,
    Tooltip,
  } from "chart.js";
  import { Radar } from "svelte-chartjs";
  import { notNull } from "svelte-utils";
  import ODStats from "../assess/ODStats.svelte";
  import { tierColors } from "../colors";
  import { Modal, percent as percent2 } from "../common";
  import { backend, slowStats, stats } from "../stores";
  import type { Stats } from "../types";
  import Attractiveness from "./Attractiveness.svelte";
  import Coherence from "./Coherence.svelte";
  import Comfort from "./Comfort.svelte";
  import Directness from "./Directness.svelte";
  import NetworkLengths from "./NetworkLengths.svelte";
  import Safety from "./Safety.svelte";

  let showSafety = false;
  let showDirectness = false;
  let showCoherence = false;
  let showComfort = false;
  let showAttractiveness = false;

  ChartJS.register(
    Title,
    Tooltip,
    Legend,
    PointElement,
    RadialLinearScale,
    LineElement,
  );

  type Rating = "very poor" | "poor" | "medium" | "good" | "very good";
  let ratingColors = {
    "very poor": "#d73027",
    poor: "#fc8d59",
    medium: "#fee08b",
    good: "#d9ef8b",
    "very good": "#1a9850",
  };

  function percent(pct: number): string {
    return `${Math.round(pct * 100)}%`;
  }

  function percent3(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }

    return Math.round((x / total) * 100);
  }

  function stepLessThanOrEqual(pct: number, steps: number[]): Rating {
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

  function stepGreaterThan(pct: number, steps: number[]): Rating {
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

  function renderScore(pair: [string, Rating]): string {
    return `<td style="background: ${ratingColors[pair[1]]}">${pair[0]} (${pair[1]})</td>`;
  }

  function radarScore(pair: [string, Rating]): number {
    // TODO Exact values
    return {
      "very poor": 0,
      poor: 20,
      medium: 40,
      good: 60,
      "very good": 80,
    }[pair[1]];
  }

  function safety(s: Stats): [string, Rating] {
    let pct = percent3(
      s.total_high_los_arterial_roads_length,
      s.total_arterial_road_length,
    );
    return [`${pct}%`, stepLessThanOrEqual(pct, [20, 40, 60, 80])];
  }

  function coherentDensity(s: Stats): [string, Rating] {
    if (!s.density_network_in_settlements) {
      return ["no routes", "very poor"];
    }
    let rating = stepGreaterThan(
      s.density_network_in_settlements,
      [1000, 500, 400, 250],
    );
    return [`${Math.round(s.density_network_in_settlements)}m`, rating];
  }

  function comfort(s: Stats): [string, Rating] {
    let pct = percent3(s.total_low_gradient_length, s.total_network_length);
    return [`${pct}%`, stepLessThanOrEqual(pct, [10, 20, 40, 60])];
  }

  function attractiveness(s: Stats): [string, Rating] {
    let pct = percent3(s.total_attractive_length, s.total_network_length);
    // First threshold will almost never happen; this is a deliberate choice
    return [`${pct}%`, stepLessThanOrEqual(pct, [0, 25, 50, 75])];
  }

  function directness(s: {
    average_weighted_directness: number;
  }): [string, Rating] {
    let rating = stepGreaterThan(
      s.average_weighted_directness,
      [1.5, 1.4, 1.3, 1.2],
    );
    return [`${s.average_weighted_directness.toFixed(1)}x`, rating];
  }
</script>

{#if $stats && $slowStats}
  {#await notNull($backend).getBaselineStats() then baseline}
    <h3>Network metrics</h3>

    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Metric</th>
          <th scope="col">Existing network quality</th>
          <th scope="col">Proposed network quality</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row">
            Safety <button
              type="button"
              class="ds_link"
              on:click={() => (showSafety = true)}
            >
              <i class="fa-solid fa-circle-info"></i>
            </button>
          </th>
          {@html renderScore(safety(baseline))}
          {@html renderScore(safety($stats))}
        </tr>

        <tr>
          <th scope="row">
            Directness <button
              type="button"
              class="ds_link"
              on:click={() => (showDirectness = true)}
            >
              <i class="fa-solid fa-circle-info"></i>
            </button>
          </th>
          {@html renderScore(directness(baseline))}
          {@html renderScore(directness($slowStats))}
        </tr>

        <tr>
          <th scope="row">
            Coherence <button
              type="button"
              class="ds_link"
              on:click={() => (showCoherence = true)}
            >
              <i class="fa-solid fa-circle-info"></i>
            </button>
          </th>
          {@html renderScore(coherentDensity(baseline))}
          {@html renderScore(coherentDensity($stats))}
        </tr>

        <tr>
          <th scope="row">
            Comfort <button
              type="button"
              class="ds_link"
              on:click={() => (showComfort = true)}
            >
              <i class="fa-solid fa-circle-info"></i>
            </button>
          </th>
          {@html renderScore(comfort(baseline))}
          {@html renderScore(comfort($stats))}
        </tr>

        <tr>
          <th scope="row">
            Attractiveness <button
              type="button"
              class="ds_link"
              on:click={() => (showAttractiveness = true)}
            >
              <i class="fa-solid fa-circle-info"></i>
            </button>
          </th>
          {@html renderScore(attractiveness(baseline))}
          {@html renderScore(attractiveness($stats))}
        </tr>
      </tbody>
    </table>

    <div style:height="500px">
      <Radar
        data={{
          labels: [
            "Safety",
            "Directness",
            "Coherence",
            "Comfort",
            "Attractiveness",
          ],
          datasets: [
            {
              label: "Existing",
              data: [
                radarScore(safety(baseline)),
                radarScore(directness(baseline)),
                radarScore(coherentDensity(baseline)),
                radarScore(comfort(baseline)),
                radarScore(attractiveness(baseline)),
              ],
              backgroundColor: "red",
            },
            {
              label: "Proposed",
              data: [
                radarScore(safety($stats)),
                radarScore(directness($slowStats)),
                radarScore(coherentDensity($stats)),
                radarScore(comfort($stats)),
                radarScore(attractiveness($stats)),
              ],
              backgroundColor: "blue",
            },
          ],
        }}
      />
    </div>

    <h3>Network components</h3>
    <NetworkLengths />

    <h3>Primary metrics</h3>

    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Metric</th>
          <th scope="col">Existing network quality</th>
          <th scope="col">Proposed network quality</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row" style:background={tierColors.Primary}>
            High cycling demand corridor coverage
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
            Arterial road network coverage
          </th>
          <td>
            {percent2(
              baseline.covered_arterial_road_length,
              baseline.total_arterial_road_length,
            )}
          </td>
          <td>
            {percent2(
              $stats.covered_arterial_road_length,
              $stats.total_arterial_road_length,
            )}
          </td>
        </tr>
      </tbody>
    </table>

    <h3>Secondary metrics</h3>

    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Metric</th>
          <th scope="col">Existing network quality</th>
          <th scope="col">Proposed network quality</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row" style:background={tierColors.Secondary}>
            Medium cycling demand corridor coverage
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
            Connected deprived population
          </th>
          <td>{percent(baseline.percent_reachable_imd_population)}</td>
          <td>{percent($stats.percent_reachable_imd_population)}</td>
        </tr>

        <tr>
          <th scope="row" style:background={tierColors.Secondary}>
            Connected population
          </th>
          <td>{percent(baseline.percent_reachable_population)}</td>
          <td>{percent($stats.percent_reachable_population)}</td>
        </tr>
      </tbody>
    </table>

    <h3>Local access metrics</h3>

    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Metric</th>
          <th scope="col">Existing network quality</th>
          <th scope="col">Proposed network quality</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row" style:background={tierColors.LocalAccess}>
            Railway stations
          </th>
          <td>{percent(baseline.percent_reachable_railway_stations)}</td>
          <td>{percent($stats.percent_reachable_railway_stations)}</td>
        </tr>

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
      </tbody>
    </table>

    <h3>Long distance metrics</h3>

    <table class="ds_table">
      <thead>
        <tr>
          <th scope="col">Metric</th>
          <th scope="col">Existing network quality</th>
          <th scope="col">Proposed network quality</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th scope="row" style:background={tierColors.LongDistance}>
            Settlements connected to the network
          </th>
          <td>{percent(baseline.percent_reachable_settlements)}</td>
          <td>{percent($stats.percent_reachable_settlements)}</td>
        </tr>
      </tbody>
    </table>
  {/await}
{/if}

<h3>Network impacts on demand</h3>
<p>
  This shows how journeys from census demand data would pick quiet routes using
  the proposed network.
</p>
<ODStats />

<Modal bind:show={showSafety}>
  <Safety />
</Modal>

<Modal bind:show={showDirectness}>
  <Directness />
</Modal>

<Modal bind:show={showCoherence}>
  <Coherence />
</Modal>

<Modal bind:show={showComfort}>
  <Comfort />
</Modal>

<Modal bind:show={showAttractiveness}>
  <Attractiveness />
</Modal>
