<script lang="ts">
  import { onMount } from "svelte";
  import { notNull } from "svelte-utils";
  import { infraTypeColors, levelOfServiceColors, tierColors } from "../colors";
  import {
    backend,
    lastUpdateOD,
    mode,
    mutationCounter,
    odStats,
  } from "../stores";

  onMount(async () => {
    if ($lastUpdateOD != $mutationCounter) {
      $odStats = await $backend!.recalculateODStats();
      $lastUpdateOD = $mutationCounter;
    }
  });
</script>

{#if $odStats && $lastUpdateOD == $mutationCounter}
  <p>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a
      href="#"
      on:click|preventDefault={() =>
        ($mode = {
          kind: "evaluate-journey",
          browse: notNull($odStats).worst_directness_routes,
        })}
    >
      Average weighted directness
    </a>
    : {$odStats.average_weighted_directness.toFixed(1)}x
  </p>

  <div class="ds_accordion" data-module="ds-accordion">
    <div class="ds_accordion-item">
      <input
        type="checkbox"
        class="visually-hidden ds_accordion-item__control"
        id="panel-1"
        aria-labelledby="panel-1-heading"
      />
      <div class="ds_accordion-item__header">
        <h2 id="panel-1-heading" class="ds_accordion-item__title">
          Percent of demand by infrastructure type
        </h2>
        <span class="ds_accordion-item__indicator"></span>
        <label class="ds_accordion-item__label" for="panel-1">
          <span class="visually-hidden">Show this section</span>
        </label>
      </div>
      <div class="ds_accordion-item__body">
        <ul>
          {#each Object.entries($odStats.od_percents_infra_type).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
            <li>
              <span style:color={infraTypeColors[key] || "black"}>{key}</span>
              : {Math.round(pct * 100)}%
            </li>
          {/each}
        </ul>
      </div>
    </div>

    <div class="ds_accordion-item">
      <input
        type="checkbox"
        class="visually-hidden ds_accordion-item__control"
        id="panel-2"
        aria-labelledby="panel-2-heading"
      />
      <div class="ds_accordion-item__header">
        <h2 id="panel-2-heading" class="ds_accordion-item__title">
          Percent of demand by tier
        </h2>
        <span class="ds_accordion-item__indicator"></span>
        <label class="ds_accordion-item__label" for="panel-2">
          <span class="visually-hidden">Show this section</span>
        </label>
      </div>
      <div class="ds_accordion-item__body">
        <ul>
          {#each Object.entries($odStats.od_percents_tier).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
            <li>
              <span style:color={tierColors[key] || "black"}>{key}</span>
              : {Math.round(pct * 100)}%
            </li>
          {/each}
        </ul>
      </div>
    </div>

    <div class="ds_accordion-item">
      <input
        type="checkbox"
        class="visually-hidden ds_accordion-item__control"
        id="panel-3"
        aria-labelledby="panel-3-heading"
      />
      <div class="ds_accordion-item__header">
        <h2 id="panel-3-heading" class="ds_accordion-item__title">
          Percent of demand by level of service
        </h2>
        <span class="ds_accordion-item__indicator"></span>
        <label class="ds_accordion-item__label" for="panel-3">
          <span class="visually-hidden">Show this section</span>
        </label>
      </div>
      <div class="ds_accordion-item__body">
        <ul>
          {#each Object.entries($odStats.od_percents_los).toSorted((a, b) => b[1] - a[1]) as [key, pct]}
            <li>
              <span style:color={levelOfServiceColors[key] || "black"}>
                {key}
              </span>
              : {Math.round(pct * 100)}%
            </li>
          {/each}
        </ul>
      </div>
    </div>
  </div>
{:else}
  <p>Recalculating...</p>
{/if}
