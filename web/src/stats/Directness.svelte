<script lang="ts">
  import { subpage } from "../assess";
  import { currentStage, mode, slowStats } from "../stores";
  import { directness } from "./";

  export let show: boolean;

  $: [value, rating] = directness($slowStats!);

  function checkJourney() {
    $mode = {
      kind: "evaluate-journey",
      browse: $slowStats!.worst_directness_routes,
    };
    show = false;
  }

  function checkNetwork() {
    $mode = { kind: "main" };
    $currentStage = "assessment";
    $subpage = "directness-network";
    show = false;
  }
</script>

<h2>Directness</h2>

<h3>Definition</h3>
<p>
  Cycling routes should follow direct, desire-line-aligned paths that minimise
  detours and delays, enabling efficient and intuitive travel.
</p>

<h3>Methodology</h3>
<p>
  We assess <b>Detour Factor</b>
  , which compares the actual cycling route distance between town centres to the
  equivalent
  <b>arterial road distance</b>
  , to evaluate how direct the network is.
</p>

<h3>How to improve</h3>
<ul>
  <li>
    Reroute along <b>shorter, straighter paths</b>
  </li>
  <li>
    Prioritise alignments that <b>follow or cut through arterial roads</b>
    , aiming for
    <b>shorter travel distances than the arterial road alternative</b>
  </li>
</ul>

<button class="ds_button ds_button--secondary" on:click={checkJourney}>
  Check journeys used to calculate directness
</button>
<button class="ds_button ds_button--secondary" on:click={checkNetwork}>
  Check directness network
</button>

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
      <td>Detour factor</td>
      <td class:match={rating == "very poor"}>&ge; 1.3</td>
      <td class:match={rating == "poor"}>&ge; 1.2</td>
      <td class:match={rating == "medium"}>&ge; 1.1</td>
      <td class:match={rating == "good"}>&ge; 1</td>
      <td class:match={rating == "very good"}>&lt; 1</td>
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
