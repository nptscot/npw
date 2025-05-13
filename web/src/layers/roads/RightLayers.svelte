<script lang="ts">
  import { backgroundLayer, currentStage, mode } from "../../stores";
  import { lastBackgroundLayer } from "../stores";
  import Toggle from "./Toggle.svelte";

  $: showRelevant = $mode.kind == "main" || $mode.kind == "edit-route";

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "s") {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT" && tag != "TEXTAREA" && tag != "SELECT") {
        e.preventDefault();

        if ($backgroundLayer == "off") {
          $backgroundLayer = $lastBackgroundLayer;
        } else {
          $lastBackgroundLayer = $backgroundLayer;
          $backgroundLayer = "off";
        }
      }
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<div class="panel">
  <h3>Background layers</h3>
  <ul>
    <Toggle name="Coherent network" style="cn" icon="fa-person-biking" />
    <Toggle
      name="Existing infrastructure"
      style="existing_infra"
      icon="fa-road"
    />
    <Toggle name="Estimated traffic volume" style="traffic" icon="fa-car" />
    <Toggle
      name="Estimated speed limit"
      style="speed"
      icon="fa-gauge-simple-high"
    />
    <Toggle name="Gradient" style="gradient" icon="fa-mound" />
    <Toggle
      name="Street space"
      style="street_space"
      icon="fa-ruler-horizontal"
    />
    <Toggle name="Attractive streets" style="attractive" icon="fa-tree" />
    <Toggle
      name="NPT full network"
      style="precalculated_rnet"
      icon="fa-diagram-project"
      relevant={showRelevant &&
        ($currentStage == "Primary" || $currentStage == "Secondary")}
    />
    <Toggle
      name="Population"
      style="population"
      icon="fa-person"
      relevant={showRelevant && $currentStage == "Secondary"}
    />
    <Toggle
      name="Deprived population (SIMD)"
      style="deprived"
      icon="fa-house-user"
      relevant={showRelevant && $currentStage == "Secondary"}
    />
  </ul>

  {#if $mode.kind != "setup"}
    <h3>Evaluation layers</h3>
    <ul>
      <Toggle name="Level of Service" style="los" icon="fa-face-smile" />
      <Toggle
        name="Severances"
        style="reachability"
        icon="fa-link"
        relevant={showRelevant &&
          ($currentStage == "Secondary" || $currentStage == "LocalAccess")}
      />
    </ul>
  {/if}
</div>

<style>
  .panel {
    position: absolute;
    width: 165px;
    top: 10px;
    right: 10px;
    background-color: white;
    padding: 10px;
  }
  .panel h3 {
    font-size: 14px;
  }
  .panel :global(ul) {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  .panel :global(ul li) {
    margin: 0 0 6px;
    padding: 0;
    line-height: 14px;
  }
  .panel :global(ul li button) {
    width: 100%;
    border: 0;
    border-left: 5px solid #ddd;
    background-color: #f7f7f7;
    text-align: left;
    padding: 4px 8px;
  }
  .panel :global(button) {
    background-color: #ccc;
    font-size: 12px;
    height: 36px;
  }
  .panel :global(button.selected),
  .panel :global(button.selected i) {
    color: #603;
    border-color: #603;
    background-color: #dcdcdc;
  }
  .panel :global(button.relevant) {
    border-left-color: #666;
  }
  .panel :global(button i) {
    display: block;
    font-size: 16px;
    width: 20px;
    color: #aaa;
    float: right;
    line-height: 26px;
  }
</style>
