<script lang="ts">
  import { stats } from "../stores";
  import { attractiveness } from "./";

  $: [value, rating] = attractiveness($stats!);
</script>

<h2>Attractiveness</h2>

<h3>Definition</h3>
<p>
  Infrastructure should feel pleasant, safe, and in harmony with its
  surroundings. Aesthetic quality and personal security matter.
</p>

<h3>Methodology</h3>
<p>
  Measured by the <b>percent of attractive streets</b>
  over the drawn network. Attractive streets are defined as being within 10m of any
  greenspace defined by Ordnance Survey Open Greenspace. This dataset does not capture
  all greenspaces, so do not prioritise this metric.
</p>
<p><i>Additional indicators like lighting and noise may be added later.</i></p>

<h3>How to improve</h3>
<ul>
  <li>
    Route through <b>attractive streets</b>
    as indicated on the right
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
      <td>share of green spaces</td>
      <td class:match={rating == "very poor"}>0</td>
      <td class:match={rating == "poor"}>&le; 25%</td>
      <td class:match={rating == "medium"}>&le; 50%</td>
      <td class:match={rating == "good"}>&le; 75%</td>
      <td class:match={rating == "very good"}>&gt; 75%</td>
      <td>{value}</td>
      <td>100%</td>
    </tr>
  </tbody>
</table>

<style>
  .match {
    font-weight: bold;
  }
</style>
