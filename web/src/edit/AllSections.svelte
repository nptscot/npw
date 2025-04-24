<script lang="ts">
  import { tierLabels } from "../colors";
  import { Modal, percent } from "../common";
  import { editModeBreakdown } from "../stores";
  import { infraTypeMapping, type AutosplitRoute, type Tier } from "../types";
  import PickInfraType from "./PickInfraType.svelte";
  import SectionDiagram from "./SectionDiagram.svelte";

  export let sectionsGj: AutosplitRoute;
  export let infraType: string;
  export let overrideInfraType: boolean;
  export let tier: Tier;

  let showOverrideModal = false;

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
  <h4>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a
      href="#"
      on:click|preventDefault={() => ($editModeBreakdown = "infra_type")}
      class:focused={$editModeBreakdown == "infra_type"}
    >
      Infrastructure type
    </a>
  </h4>

  <SectionDiagram breakdown="infra_type" {sectionsGj} />

  {#if overrideInfraType}
    <p>
      You've forced this route to always use {infraTypeMapping[infraType][0]}.
    </p>

    <button
      class="ds_button ds_button--secondary"
      on:click={() => (overrideInfraType = false)}
    >
      Remove override
    </button>
  {:else}
    <p>
      The route you've drawn has been split into sections, automatically picking
      an infrastructure type to achieve the best possible Level of Service.
    </p>

    <button
      class="ds_button ds_button--secondary"
      on:click={() => {
        overrideInfraType = true;
        showOverrideModal = true;
      }}
    >
      Override infrastructure type
    </button>
  {/if}
</section>

<section>
  <!-- svelte-ignore a11y-invalid-attribute -->
  <h4>
    <a
      href="#"
      on:click|preventDefault={() => ($editModeBreakdown = "deliverability")}
      class:focused={$editModeBreakdown == "deliverability"}
    >
      Deliverability
    </a>
  </h4>

  <SectionDiagram breakdown="deliverability" {sectionsGj} />

  {#if pctFits != "100%"}
    <p>
      Only {pctFits} of the route fits in the available streetspace. You may need
      to override the infrastructure type for some sections.
    </p>
  {/if}
</section>

<section>
  <h4>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a
      href="#"
      on:click|preventDefault={() => ($editModeBreakdown = "los")}
      class:focused={$editModeBreakdown == "los"}
    >
      Level of Service
    </a>
  </h4>

  <SectionDiagram breakdown="los" {sectionsGj} />

  {#if pctHighLoS != "100%"}
    <p>
      Only {pctHighLoS} of the route has a high level of service. You may need to
      override the infrastructure type for some sections and reduce traffic speeds
      and volumes.
    </p>
  {/if}
</section>

<section>
  <h4>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a
      href="#"
      on:click|preventDefault={() => ($editModeBreakdown = "tier")}
      class:focused={$editModeBreakdown == "tier"}
    >
      Tier
    </a>
  </h4>

  <SectionDiagram breakdown="tier" {sectionsGj} />

  {#if pctMatchesTier != "100%"}
    <p>
      Only {pctMatchesTier} of the route will use the {tierLabels[tier]} tier. Because
      the route enters and exits settlements, part of it is assigned to a different
      tier.
    </p>
  {/if}
</section>

<section>
  <h4>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a
      href="#"
      on:click|preventDefault={() => ($editModeBreakdown = "gradient")}
      class:focused={$editModeBreakdown == "gradient"}
    >
      Gradient
    </a>
  </h4>

  <SectionDiagram breakdown="gradient" {sectionsGj} />
</section>

<Modal bind:show={showOverrideModal}>
  <PickInfraType
    onFinish={(value) => {
      infraType = value;
      showOverrideModal = false;
    }}
  />
</Modal>

<style>
  .focused {
    text-decoration: underline;
  }
</style>
