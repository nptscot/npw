<script lang="ts">
  import type { RouteGJ, Step } from "./stores";

  export let gj: RouteGJ;

  let byInfraType = (step: Step) => step.infra_type;
  let byLos = (step: Step) => step.los;

  function numChanges(gj: RouteGJ, key: (step: Step) => string): number {
    let count = 0;
    // No windows(2)?
    for (let i = 0; i < gj.directions.length - 1; i++) {
      let x1 = key(gj.directions[i]);
      let x2 = key(gj.directions[i + 1]);
      if (x1 != x2) {
        count++;
      }
    }
    return count;
  }

  function percentages(gj: RouteGJ, key: (step: Step) => string): string {
    let lengthByKey: { [name: string]: number } = {};
    for (let step of gj.directions) {
      let x = key(step);
      if (!Object.hasOwn(lengthByKey, x)) {
        lengthByKey[x] = 0;
      }
      lengthByKey[x] += step.length;
    }
    let total = 0;
    for (let length of Object.values(lengthByKey)) {
      total += length;
    }

    let results = [];
    for (let [x, length] of Object.entries(lengthByKey).toSorted(
      (a, b) => b[1] - a[1],
    )) {
      let percent = Math.round((length / total) * 100);
      results.push(`${percent}% ${x}`);
    }
    return results.join(", ");
  }
</script>

<p>
  Detour factor: <b>{(gj.route_length / gj.direct_length).toFixed(1)}x</b>
  longer than straight line
</p>
<p>
  <b>{(gj.route_length / gj.car_length).toFixed(1)}x</b>
  longer than the driving route (in
  <span style:color="red">red</span>
  )
</p>

<hr />

<p>{numChanges(gj, byInfraType)} changes in infrastructure type</p>
<p>By length: {percentages(gj, byInfraType)}</p>

<hr />

<p>{numChanges(gj, byLos)} changes in level of service</p>
<p>By length: {percentages(gj, byLos)}</p>

<hr />

<details>
  <summary>Route directions</summary>

  <ol>
    {#each gj.directions as step}
      <li>
        <a href={step.way} target="_blank">[{step.infra_type}] {step.name}</a>
      </li>
    {/each}
  </ol>
</details>
