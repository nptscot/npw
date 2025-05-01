<script lang="ts">
  import { tierLabels } from "../colors";
  import { percent } from "../common";
  import { editModeBreakdown } from "../stores";
  import { type AutosplitRoute, type Tier } from "../types";
  import SectionDiagram from "./SectionDiagram.svelte";

  export let sectionsGj: AutosplitRoute;
  export let tier: Tier;

  $: pctFits = percentFits(sectionsGj);
  $: pctHighLoS = percentHighLoS(sectionsGj);
  $: pctMatchesTier = percentMatchesTier(sectionsGj);

  function percentFits(sectionsGj: AutosplitRoute): string {
    let total = 0;
    let fits = 0;
    for (let f of sectionsGj.features) {
      total += f.properties.length;
      if (f.properties.fits) {
        fits += f.properties.length;
      }
    }
    return percent(fits, total);
  }

  function percentHighLoS(sectionsGj: AutosplitRoute): string {
    let total = 0;
    let high = 0;
    for (let f of sectionsGj.features) {
      total += f.properties.length;
      if (f.properties.los == "High") {
        high += f.properties.length;
      }
    }
    return percent(high, total);
  }

  function percentMatchesTier(sectionsGj: AutosplitRoute): string {
    let total = 0;
    let matches = 0;
    for (let f of sectionsGj.features) {
      total += f.properties.length;
      if (f.properties.tier == tier) {
        matches += f.properties.length;
      }
    }
    return percent(matches, total);
  }
</script>

<section>
  <div style="display: flex; justify-content: space-between">
    <b>Infrastructure type</b>
    <button
      class:style-icon={true}
      on:click={() => ($editModeBreakdown = "infra_type")}
      class:focused={$editModeBreakdown == "infra_type"}
    >
      ☰
    </button>
  </div>

  <SectionDiagram breakdown="infra_type" {sectionsGj} />
</section>

<section>
  <div style="display: flex; justify-content: space-between">
    <b>Deliverability</b>
    <span>
      <span>{pctFits} fits</span>
      <button
        class:style-icon={true}
        on:click={() => ($editModeBreakdown = "deliverability")}
        class:focused={$editModeBreakdown == "deliverability"}
      >
        <i class="fa-solid fa-person-digging"></i>
      </button>
    </span>
  </div>

  <SectionDiagram breakdown="deliverability" {sectionsGj} />
</section>

<section>
  <div style="display: flex; justify-content: space-between">
    <b>Level of Service</b>
    <span>
      <span>{pctHighLoS} high</span>
      <button
        class:style-icon={true}
        on:click={() => ($editModeBreakdown = "los")}
        class:focused={$editModeBreakdown == "los"}
      >
        <i class="fa-solid fa-face-smile"></i>
      </button>
    </span>
  </div>

  <SectionDiagram breakdown="los" {sectionsGj} />
</section>

<section>
  <div style="display: flex; justify-content: space-between">
    <b>Tier</b>
    <span>
      <span>{pctMatchesTier} uses {tierLabels[tier]}</span>
      <button
        class:style-icon={true}
        on:click={() => ($editModeBreakdown = "tier")}
        class:focused={$editModeBreakdown == "tier"}
      >
        ‖‖
      </button>
    </span>
  </div>

  <SectionDiagram breakdown="tier" {sectionsGj} />
</section>

<section>
  <div style="display: flex; justify-content: space-between">
    <b>Gradient</b>
    <button
      class:style-icon={true}
      on:click={() => ($editModeBreakdown = "gradient")}
      class:focused={$editModeBreakdown == "gradient"}
    >
      <i class="fa-solid fa-mound"></i>
    </button>
  </div>

  <SectionDiagram breakdown="gradient" {sectionsGj} />
</section>

<style>
  .style-icon {
    background-color: #eee;
    margin: 4px;
  }

  .focused {
    font-weight: bold;
    background-color: #aaa;
  }
</style>
