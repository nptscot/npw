<script lang="ts">
  import { GeoJSON, hoverStateFilter, CircleLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { backend, percent, type Schools } from "../stores";
  import { QualitativeLegend } from "../common";

  let show = false;

  let data: Schools = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.getSchools();
    }
  }

  $: if (show && data.features.length == 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={show} />
    Schools
  </label>

  {#if show}
    <button on:click={recalc}>Recalculate</button>
    <p>
      {reachable.toLocaleString()} / {data.features.length.toLocaleString()} ({percent(
        reachable,
        data.features.length,
      )}) reachable
    </p>
    <QualitativeLegend
      colors={{ Reachable: "purple", "Not reachable": "red" }}
    />
  {/if}
</LayerControls>

<GeoJSON {data} generateId>
  <CircleLayer
    manageHoverState
    paint={{
      "circle-color": ["case", ["get", "reachable"], "purple", "red"],
      "circle-radius": hoverStateFilter(5, 8),
    }}
    layout={{
      visibility: show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      {props.name} is a {props.kind} school with {props.pupils} pupils. It {props.reachable
        ? "is"
        : "is not"} reachable.
    </Popup>
  </CircleLayer>
</GeoJSON>
