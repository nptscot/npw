<script lang="ts">
  import type { RouteGJ } from "./stores";

  export let gj: RouteGJ;

  function infraTypeChanges(gj: RouteGJ): number {
    let count = 0;
    // No windows(2)?
    for (let i = 0; i < gj.directions.length - 1; i++) {
      let t1 = gj.directions[i].infra_type;
      let t2 = gj.directions[i + 1].infra_type;
      if (t1 != t2) {
        count++;
      }
    }
    return count;
  }

  function infraTypePercentages(gj: RouteGJ): string {
    let lengthByType: { [name: string]: number } = {};
    for (let step of gj.directions) {
      if (!Object.hasOwn(lengthByType, step.infra_type)) {
        lengthByType[step.infra_type] = 0;
      }
      lengthByType[step.infra_type] += step.length;
    }
    console.log(lengthByType);
    let total = 0;
    for (let length of Object.values(lengthByType)) {
      total += length;
    }

    let results = [];
    for (let [infraType, length] of Object.entries(lengthByType)) {
      let percent = Math.round((length / total) * 100);
      results.push(`${percent}% ${infraType}`);
    }
    return results.join(", ");
  }
</script>

<p>
  Detour factor: <b>{(gj.route_length / gj.direct_length).toFixed(1)}x</b>
  longer than straight line
</p>

<p>{infraTypeChanges(gj)} changes in infrastructure type</p>
<p>By length: {infraTypePercentages(gj)}</p>

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
