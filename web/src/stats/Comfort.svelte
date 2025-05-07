<script lang="ts">
  import { stats } from "../stores";
  import { comfort } from "./";

  $: [value, rating] = comfort($stats!);
</script>

<h2>Comfort</h2>

<h3>Definition</h3>
<p>
  Cycling should be smooth, easy, and stress-free—minimising both physical
  effort and mental strain.
</p>

<h3>Methodology</h3>
<p>
  Currently measured by the <b>proportion of low-gradient segments</b>
  (&le; 3%), as gradient significantly affects comfort.
</p>
<p>
  <i>
    Due to current data limitations, only this metric is used. Additional
    comfort-related indicators will be introduced as data becomes available.
  </i>
</p>

<h3>How to improve</h3>
<ul>
  <li>
    Plan routes with <b>gentle slopes</b>
    and avoid steep gradients wherever possible
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
      <td>Proportion of low gradient (&lt; 3%) length</td>
      <td class:match={rating == "very poor"}>&le; 10%</td>
      <td class:match={rating == "poor"}>&le; 20%</td>
      <td class:match={rating == "medium"}>&le; 40%</td>
      <td class:match={rating == "good"}>&le; 60%</td>
      <td class:match={rating == "very good"}>&gt; 80%</td>
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
