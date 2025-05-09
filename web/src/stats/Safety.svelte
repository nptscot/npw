<script lang="ts">
  import { stats } from "../stores";
  import { safetyArterial, safetyPrimarySecondary } from "./";

  $: [arterialValue, arterialRating] = safetyArterial($stats!);
  $: [primarySecondaryValue, primarySecondaryRating] = safetyPrimarySecondary(
    $stats!,
  );
</script>

<h2>Safety</h2>

<h3>Definition</h3>
<p>
  Cycle routes should minimise collision risks. Users should feel and be safe
  when cycling.
</p>

<h3>Methodology</h3>
<p>
  We use the <b>Safety Level of Service (LoS)</b>
  as a key indicator to evaluate network safety across the network.
</p>

<h3>How to improve</h3>
<ul>
  <li>
    Draw high LoS routes along <b>arterial roads</b>
  </li>
  <li>
    Override infrastructure types by adding <b>traffic constraints</b>
    , e.g. reduced speed limits or segregated lanes
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
      <td>% of high LoS among arterial roads</td>
      <td class:match={arterialRating == "very poor"}>&le; 20%</td>
      <td class:match={arterialRating == "poor"}>&le; 40%</td>
      <td class:match={arterialRating == "medium"}>&le; 60%</td>
      <td class:match={arterialRating == "good"}>&le; 80%</td>
      <td class:match={arterialRating == "very good"}>&gt; 80%</td>
      <td>{arterialValue}</td>
      <td>90%</td>
    </tr>
    <tr>
      <td>% of high LoS among primary/secondary network</td>
      <td class:match={primarySecondaryRating == "very poor"}>&le; 20%</td>
      <td class:match={primarySecondaryRating == "poor"}>&le; 40%</td>
      <td class:match={primarySecondaryRating == "medium"}>&le; 60%</td>
      <td class:match={primarySecondaryRating == "good"}>&le; 80%</td>
      <td class:match={primarySecondaryRating == "very good"}>&gt; 80%</td>
      <td>{primarySecondaryValue}</td>
      <td>10%</td>
    </tr>
  </tbody>
</table>

<style>
  .match {
    font-weight: bold;
  }
</style>
