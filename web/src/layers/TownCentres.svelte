<script lang="ts">
  import { GeoJSON, hoverStateFilter, FillLayer } from "svelte-maplibre";
  import { Popup } from "svelte-utils/map";
  import LayerControls from "./LayerControls.svelte";
  import { backend, type TownCentres } from "../stores";
  import { percent } from "../utils";
  import { QualitativeLegend } from "../common";
  import { townCentres as show } from "./stores";

  let data: TownCentres = {
    type: "FeatureCollection",
    features: [],
  };

  async function recalc() {
    if ($backend) {
      data = await $backend.getTownCentres();
    }
  }

  $: if ($show && data.features.length == 0) {
    recalc();
  }

  $: reachable = data.features.filter((f) => f.properties.reachable).length;
</script>

<LayerControls>
  <label>
    <input type="checkbox" bind:checked={$show} />
    Town centres
  </label>

  {#if $show}
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
  <FillLayer
    manageHoverState
    paint={{
      "fill-color": ["case", ["get", "reachable"], "purple", "red"],
      "fill-opacity": hoverStateFilter(0.7, 0.9),
    }}
    layout={{
      visibility: $show ? "visible" : "none",
    }}
  >
    <Popup openOn="hover" let:props>
      Town centre {props.name || ""}
      {props.reachable ? "is" : "is not"} reachable.
    </Popup>
  </FillLayer>
</GeoJSON>
