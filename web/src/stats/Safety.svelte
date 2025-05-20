<script lang="ts">
  import { stats } from "../stores";
  import { safetyArterial, safetyCombined, safetyPrimarySecondary } from "./";

  $: [arterialValue, arterialRating] = safetyArterial($stats!);
  $: [primarySecondaryValue, primarySecondaryRating] = safetyPrimarySecondary(
    $stats!,
  );
  $: [combinedValue, combinedRating] = safetyCombined($stats!);
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
  as a key indicator to evaluate network safety. LoS depends on the infrastructure
  type, the speed limit, estimated traffic volume, and whether a route is within
  a settlement, using guidance from Cycling by Design. The sub-metrics measure the
  percent of high LoS by length among all arterial roads and among the drawn primary/secondary
  network. The second sub-metric might be a high percent when the primary/secondary
  network is very small, so the value is not meaningful until you draw your network.
</p>

<h3>How to improve</h3>
<ul>
  <li>
    Draw high LoS routes along <b>arterial roads</b>
  </li>
  <li>
    Find sections of your drawn network that do not achieve high LoS using the
    bottom panel, then override the infrastructure type to include <b>
      traffic measures
    </b>
    like speed limits, bus gates, and turn restrictions
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
