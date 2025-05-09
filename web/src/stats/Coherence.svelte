<script lang="ts">
  import { stats } from "../stores";
  import {
    coherenceCombined,
    coherenceDensity,
    coherenceIntegrity,
    safetyArterial,
  } from "./";

  $: [arterialValue, arterialRating] = safetyArterial($stats!);
  $: [densityValue, densityRating] = coherenceDensity($stats!);
  $: [integrityValue, integrityRating] = coherenceIntegrity($stats!);
  $: [combinedValue, combinedRating] = coherenceCombined($stats!);
</script>

<h2>Coherence</h2>

<h3>Definition</h3>
<p>
  A coherence cycle network should provide continuous, well-connected routes
  that link key destinations such as homes, schools, shops, and transport hubs.
</p>

<h3>Methodology</h3>
<p>
  We assess <b>network density</b>
  ,
  <b>network integrity</b>
  （whether the network is continuous or fragmented, and
  <b>Safety Level of Service (LoS)</b>
  coverage on arterial roads.
</p>

<h3>How to improve</h3>
<ul>
  <li>
    Increase route density, especially along <b>primary and secondary routes</b>
  </li>
  <li>
    Ensure <b>high LoS coverage</b>
    on arterial roads
  </li>
  <li>
    Avoid fragmented networks — aim for a <b>single, integrated network</b>
    to support internal connectivity and intuitive navigation
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
