<script lang="ts">
  import type { Map } from "maplibre-gl";
  import {
    backend,
    backgroundLayer,
    boundaryName,
    currentStage,
    editsRoadStyle,
    map,
    mode,
    showReportProblem,
  } from "../stores";
  import { Checkbox, Modal, Radio } from "./";

  let problemType = "";
  let details = "";
  let email = "";
  let includeNetwork = false;
  let automaticDetails = {};

  $: if ($showReportProblem) {
    start();
  }

  function start() {
    automaticDetails = {
      boundary: $boundaryName,
      viewport: getViewportHash($map!),
      mode: JSON.stringify($mode),
      currentStage: $currentStage,
      editsRoadStyle: $editsRoadStyle,
      backgroundLayer: $backgroundLayer,
    };
  }

  async function submit() {
    let network = includeNetwork ? await $backend!.getAllRoutes() : null;

    // @ts-expect-error Unused
    let req = {
      problemType,
      details,
      email,
      network,
      ...automaticDetails,
    };
    window.alert("TODO: This report isn't submitted anywhere yet");

    cancel();
  }

  function cancel() {
    $showReportProblem = false;
    problemType = "";
    details = "";
    email = "";
    includeNetwork = false;
    automaticDetails = {};
  }

  // Adapted from https://github.com/maplibre/maplibre-gl-js/blob/5d7e6d52000a8569ac2308a9aef14c98933eb0d8/src/ui/hash.ts
  function getViewportHash(map: Map): string {
    let center = map.getCenter();
    let zoom = Math.round(map.getZoom() * 100) / 100;
    // derived from equation: 512px * 2^z / 360 / 10^d < 0.5px
    let precision = Math.ceil(
      (zoom * Math.LN2 + Math.log(512 / 360 / 0.5)) / Math.LN10,
    );
    let m = Math.pow(10, precision);
    let lat = Math.round(center.lat * m) / m;
    let lng = Math.round(center.lng * m) / m;
    let hash = `${zoom}/${lat}/${lng}`;

    let bearing = map.getBearing();
    let pitch = map.getPitch();
    if (bearing || pitch) {
      hash += `/${Math.round(bearing * 10) / 10}`;
    }
    if (pitch) {
      hash += `/${Math.round(pitch)}`;
    }
    return `#${hash}`;
  }
</script>

<Modal bind:show={$showReportProblem}>
  <h2>Report a problem</h2>

  <p>You can report a problem with NPW or the data to the team.</p>

  <Radio
    bind:value={problemType}
    options={[
      ["layer_wrong", "The data shown by the current layer is wrong"],
      ["missing_road", "I can't draw a route to a road or path"],
      [
        "extra_road",
        "My route snaps to a private road that should not be included",
      ],
      [
        "boundaries_wrong",
        "A town centre, settlement, or area boundary shown is wrong",
      ],
      ["other", "Other"],
    ]}
    legend="What's wrong?"
  />

  <br />

  <fieldset>
    <legend>Please describe the problem</legend>
    <textarea
      class="ds_input"
      rows="4"
      placeholder="Details"
      bind:value={details}
    />
  </fieldset>

  <fieldset>
    <legend>
      If you want to be contacted when this problem is fixed, what is your
      email? (Optional)
    </legend>
    <input
      type="email"
      class="ds_input ds_input--fixed-20"
      placeholder="Email"
      bind:value={email}
    />
  </fieldset>

  <fieldset>
    <legend>
      If your problem relates to what you have drawn, send your network?
    </legend>
    <Checkbox bind:checked={includeNetwork}>
      Include your current drawn network
    </Checkbox>
  </fieldset>

  <fieldset>
    <legend>The following information will be included in your report:</legend>
    <ul>
      {#each Object.entries(automaticDetails) as [key, value]}
        <li>{key}: {value}</li>
      {/each}
    </ul>
  </fieldset>

  <div><button class="ds_button" on:click={submit}>Submit report</button></div>

  <div>
    <button class="ds_button ds_button--secondary" on:click={cancel}>
      Cancel
    </button>
  </div>
</Modal>
