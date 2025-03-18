<script lang="ts">
  import { stats } from "../stores";

  // Returns something [0, 1]
  function percent(x: number, total: number): number {
    if (total == 0) {
      return 0;
    }
    return (100 * x) / total;
  }
</script>

{#if $stats}
  <div class="progress-summary">
    <ul>
      <li title="What percent of your network has high Level of Service?">
        Safety
        <br />
        <progress
          value={percent(
            $stats.total_high_los_length,
            $stats.total_network_length,
          )}
          max="100"
        />
      </li>

      <li title="TODO: Placeholder score">
        Directness
        <br />
        <progress value="50" max="100" />
      </li>

      <li title="Percent of main roads covered by network">
        Coherence
        <br />
        <progress
          value={percent(
            $stats.covered_main_road_length,
            $stats.total_main_road_length,
          )}
          max="100"
        />
      </li>

      <!--<li title="Density of primary/secondary network within settlements">
        Coherence (density)
        <br />
        {#if $stats.density_network_in_settlements}
          {Math.round($stats.density_network_in_settlements)}m
        {:else}
          no routes yet
        {/if}
      </li>-->

      <li title="What percent of your network is on low gradient (&le; 3%)?">
        Comfort
        <br />
        <progress
          value={percent(
            $stats.total_low_gradient_length,
            $stats.total_network_length,
          )}
          max="100"
        />
      </li>

      <li title="TODO: Placeholder score">
        Attractiveness
        <br />
        <progress value="50" max="100" />
      </li>
    </ul>
  </div>
{/if}

<style>
  /* TODO Does this do anything? Why the big <a>?
  .progress-summary a {
    text-decoration: none;
    color: #1a1a1a;
    }*/
  .progress-summary ul {
    height: 100%;
    list-style: none;
    display: flex;
    margin: 0;
    padding: 0;
  }
  .progress-summary ul li {
    list-style: none;
    font-size: 1rem;
    margin: 0;
    width: 100px;
    line-height: 12px;
    padding: 12px 6px 4px;
  }
  .progress-summary ul li progress {
    width: 100%;
  }
</style>
