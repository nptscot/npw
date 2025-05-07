<script lang="ts">
  import {
    infraTypeColors,
    infraTypeLabels,
    levelOfServiceColors,
    speedColors,
    trafficColors,
    trafficLabels,
  } from "../colors";
  import type { AutosplitRoute } from "../types";

  export let sectionsGj: AutosplitRoute;
</script>

<table class="ds_table">
  <thead>
    <tr>
      <th scope="col">Speed</th>
      <th scope="col">Traffic volume</th>
      <th scope="col">Infrastructure type</th>
      <th scope="col">Level of Service</th>
    </tr>
  </thead>
  <tbody>
    {#each sectionsGj.los_details as row}
      <tr>
        <td style:color={speedColors[row.speed] ?? "black"}>{row.speed} mph</td>
        <td style:color={trafficColors[row.traffic]}>
          {trafficLabels[row.traffic]}
        </td>
        <td style:color={infraTypeColors[row.infra_type]}>
          {infraTypeLabels[row.infra_type]}
        </td>
        <td style:color={levelOfServiceColors[row.los]}>{row.los}</td>
      </tr>
    {/each}
  </tbody>
</table>
