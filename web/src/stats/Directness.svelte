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
  We calculate an <b>average detour factor</b>
  by checking the journey route between all town centres within 5km of each other.
  For each pair of town centres, we calculate a
  <b>direct route</b>
  that minimises distance, ignoring infrastructure and Level of Service (LoS) completely.
  We also calculate a
  <b>balanced route</b>
  that penalises roads by LoS, treating a low LoS road as 3 times longer than the
  equivalent high LoS road. The ratio of the
  <b>weighted balanced route score</b>
  and the
  <b>unweighted direct route distance</b>
  gives the
  <b>detour factor</b>
  .
</p>
<p>
  When the direct and balanced route follow the same path, the detour factor
  will be 1 (the best case) only when the entire route is high LoS. In other
  cases, the balanced route will either take longer detours to find nearby
  higher LoS roads, or sometimes it is forced to go along low LoS roads.
</p>

<h3>How to improve</h3>
<ul>
  <li>
    Prioritise alignments that <b>follow arterial roads</b>
    , rather than zig-zagging through quieter residential streets
  </li>
  <li>
    Ensure the direct routes have <b>high LoS</b>
  </li>
  <li>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|preventDefault={checkJourney}>
      Check the town centres with the worst detour factors
    </a>
  </li>
  <li>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a href="#" on:click|preventDefault={checkNetwork}>
      Compare all direct routes between town centres with the balanced routes
    </a>
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
      <td>Average detour factor</td>
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
