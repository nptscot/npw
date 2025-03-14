<script lang="ts">
  import { referenceRoadStyle } from "../../stores";
  import { lastReferenceStyle } from "../stores";
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
  <Toggle name="Core network" style="cn" />
  <Toggle name="Existing infrastructure type" style="existing_infra" />
  <Toggle name="Estimated traffic volume" style="traffic" />
  <Toggle name="Gradient" style="gradient" />
  <Toggle name="Street space" style="street_space" />
  <Toggle name="Estimated speed limit" style="speed" />
  <Toggle name="NPT full network" style="precalculated_rnet" />

  <h3>Evaluation</h3>
  <Toggle name="Level of Service" style="los" />
  <Toggle name="Reachability" style="reachability" />
  <Toggle name="Route network (calculated)" style="calculated_rnet" />
  <Toggle name="Network disconnections" style="disconnections" />
  <Toggle name="Streetspace deliverability" style="deliverability" />
</div>

<style>
  .panel {
    position: absolute;
    top: 10px;
    right: 10px;
    width: 15%;

    display: flex;
    flex-direction: column;
    background: white;
  }
</style>
