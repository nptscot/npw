<script lang="ts">
  import { subpage } from "../assess";
  import { currentStage, mode, stats } from "../stores";
  import {
    coherenceCombined,
    coherenceDensity,
    coherenceIntegrity,
    safetyArterial,
  } from "./";

  export let show: boolean;

  $: [arterialValue, arterialRating] = safetyArterial($stats!);
  $: [densityValue, densityRating] = coherenceDensity($stats!);
  $: [integrityValue, integrityRating] = coherenceIntegrity($stats!);
  $: [combinedValue, combinedRating] = coherenceCombined($stats!);

  function meshDensity() {
    $mode = { kind: "main" };
    $currentStage = "assessment";
    $subpage = "mesh-density";
    show = false;
  }

  function networkSplits() {
    $mode = { kind: "main" };
    $currentStage = "assessment";
    $subpage = "disconnected";
    show = false;
  }
</script>

<h2>Coherence</h2>

<h3>Definition</h3>
<p>
  A coherent cycle network should provide continuous, well-connected routes that
  link key destinations such as homes, schools, shops, and transport hubs.
</p>

<h3>Methodology</h3>
<p>
  We assess <b>network density</b>
  by dividing the total settlement area by the total length of primary and secondary
  routes within settlements.
</p>
<p>
  <b>Safety Level of Service (LoS)</b>
  checks for high LoS on arterial roads
</p>
<p>
  <b>Network integrity</b>
  measures the number of disconnected pieces of your network. There should ideally
  be just one piece per settlement.
</p>

<h3>How to improve</h3>
<ul>
  <li>
    Draw more <b>primary and secondary routes</b>
    to increase density.
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|preventDefault={meshDensity}>Check mesh density</a>
    for areas to target.
  </li>
  <li>
    Draw high LoS routes along <b>arterial roads</b>
  </li>
  <li>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|preventDefault={networkSplits}>Check network splits</a>
    and fill in the gaps between pieces of your network.
  </li>
</ul>

<h3>Sub-metrics</h3>
<table class="ds_table">
  <thead>
    <tr>
      <th scope="col" rowspan="2">Sub-metrics</th>
      <th scope="col" colspan="5">Evaluation scale</th>
      <th scope="col" rowspan="2">Value</th>
      <th scope="col" rowspan="2">Weight</th>
    </tr>
    <tr>
      <td>1 - very poor</td>
      <td>2 - poor</td>
      <td>3 - medium</td>
      <td>4 - good</td>
      <td>5 - very good</td>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>Network density</td>
      <td class:match={densityRating == "very poor"}>&gt; 1000m</td>
      <td class:match={densityRating == "poor"}>&gt; 500m</td>
      <td class:match={densityRating == "medium"}>&gt; 400m</td>
      <td class:match={densityRating == "good"}>&gt; 250m</td>
      <td class:match={densityRating == "very good"}>&le; 250m</td>
      <td>{densityValue}</td>
      <td>40%</td>
    </tr>
    <tr>
      <td>% of high LoS among arterial roads</td>
      <td class:match={arterialRating == "very poor"}>&le; 20%</td>
      <td class:match={arterialRating == "poor"}>&le; 40%</td>
      <td class:match={arterialRating == "medium"}>&le; 60%</td>
      <td class:match={arterialRating == "good"}>&le; 80%</td>
      <td class:match={arterialRating == "very good"}>&gt; 80%</td>
      <td>{arterialValue}</td>
      <td>30%</td>
    </tr>
    {#if $stats}
      <tr>
        <td>Network integrity</td>
        <td class:match={integrityRating == "very poor"}>
          &gt; {$stats.num_settlements}
        </td>
        <td></td>
        <td class:match={integrityRating == "medium"}>
          &le; {$stats.num_settlements}
        </td>
        <td></td>
        <td class:match={integrityRating == "very good"}>1</td>
        <td>{integrityValue}</td>
        <td>30%</td>
      </tr>
    {/if}
    <tr>
      <td><b>Combined</b></td>
      <td class:match={combinedRating == "very poor"}>&le; 20%</td>
      <td class:match={combinedRating == "poor"}>&le; 40%</td>
      <td class:match={combinedRating == "medium"}>&le; 60%</td>
      <td class:match={combinedRating == "good"}>&le; 80%</td>
      <td class:match={combinedRating == "very good"}>&gt; 80%</td>
      <td>{combinedValue}</td>
      <td></td>
    </tr>
  </tbody>
</table>

<style>
  .match {
    font-weight: bold;
  }
</style>
