<script lang="ts">
  // This component must be instantiated after $stats and $slowStats are up-to-date
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
  import {
    attractiveness,
    coherentDensity,
    coherentIntegrity,
    comfort,
    directness,
    safetyArterial,
    safetyCombined,
    safetyPrimarySecondary,
    type Rating,
  } from "./";
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
          {@html renderScore(safetyCombined(baseline))}
          {@html renderScore(safetyCombined($stats))}
        </tr>
        <tr>
          <td>% of high LoS among arterial roads</td>
          {@html renderScore(safetyArterial(baseline))}
          {@html renderScore(safetyArterial($stats))}
        </tr>
        <tr>
          <td>% of high LoS among primary/secondary network</td>
          {@html renderScore(safetyPrimarySecondary(baseline))}
          {@html renderScore(safetyPrimarySecondary($stats))}
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
          <td>TODO</td>
          <td>TODO</td>
        </tr>
        <tr>
          <td>Network density</td>
          {@html renderScore(coherentDensity(baseline))}
          {@html renderScore(coherentDensity($stats))}
        </tr>
        <tr>
          <td>% of high LoS among arterial roads</td>
          {@html renderScore(safetyArterial(baseline))}
          {@html renderScore(safetyArterial($stats))}
        </tr>
        <tr>
          <td>Network integrity</td>
          {@html renderScore(coherentIntegrity(baseline))}
          {@html renderScore(coherentIntegrity($stats))}
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
                radarScore(safetyCombined(baseline)),
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
                radarScore(safetyCombined($stats)),
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
