<script lang="ts">
  import { currentStage, referenceRoadStyle } from "../../stores";
  import { lastReferenceStyle } from "../stores";
  import PopulationToggle from "./PopulationToggle.svelte";
  import Toggle from "./Toggle.svelte";

  function onKeyDown(e: KeyboardEvent) {
    if (e.key == "s") {
      let tag = (e.target as HTMLElement).tagName;
      if (tag != "INPUT") {
        e.preventDefault();

        if ($referenceRoadStyle == "off") {
          $referenceRoadStyle = $lastReferenceStyle;
        } else {
          $lastReferenceStyle = $referenceRoadStyle;
          $referenceRoadStyle = "off";
        }
      }
    }
  }
</script>

<svelte:window on:keydown={onKeyDown} />

<div class="panel">
  <h3>Background layers</h3>
  <ul>
    <Toggle name="Core network" style="cn" icon="fa-person-biking" />
    <Toggle
      name="Existing infrastructure type"
      style="existing_infra"
      icon="fa-road"
    />
    <Toggle name="Estimated traffic volume" style="traffic" icon="fa-car" />
    <Toggle name="Gradient" style="gradient" icon="fa-mound" />
    <Toggle
      name="Street space"
      style="street_space"
      icon="fa-ruler-horizontal"
    />
    <Toggle
      name="Estimated speed limit"
      style="speed"
      icon="fa-gauge-simple-high"
    />
    <Toggle
      name="NPT full network"
      style="precalculated_rnet"
      icon="fa-diagram-project"
      relevant={$currentStage == "Primary" || $currentStage == "Secondary"}
    />
  </ul>

  <h3>Evaluation layers</h3>
  <ul>
    <Toggle name="Level of Service" style="los" icon="fa-face-smile" />
    <Toggle name="Reachability" style="reachability" icon="fa-link" />
    <Toggle
      name="Route network (calculated)"
      style="calculated_rnet"
      icon="fa-globe"
    />
    <Toggle
      name="Network disconnections"
      style="disconnections"
      icon="fa-link-slash"
      relevant={$currentStage == "assessment"}
    />
    <Toggle
      name="Streetspace deliverability"
      style="deliverability"
      icon="fa-person-digging"
      relevant={$currentStage == "assessment"}
    />
    <hr />
    <PopulationToggle
      name="Population"
      style="population"
      icon="fa-person"
      relevant={$currentStage == "LocalAccess"}
    />
    <PopulationToggle
      name="Deprived population (SIMD)"
      style="deprived"
      icon="fa-house-user"
      relevant={$currentStage == "LocalAccess"}
    />
  </ul>
</div>

<style>
  .panel {
    position: absolute;
    width: 165px;
    top: 10px;
    bottom: 40px;
    right: 10px;
    background-color: white;
    overflow-y: auto;
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
