<script lang="ts">
  import { onMount } from "svelte";
  import { GeoJSON, LineLayer } from "svelte-maplibre";
  import { Loading } from "svelte-utils";
  import { constructMatchExpression } from "svelte-utils/map";
  import { levelOfServiceColors } from "../colors";
  import { layerId, roadLineWidth } from "../common";
  import { backend } from "../stores";
  import type { TownCentreRoutes } from "../types";
  import { showDirectness } from "./";

  let data: TownCentreRoutes = {
    type: "FeatureCollection",
    features: [],
  };

  let loading = "";

  // TODO Cache this
  onMount(async () => {
    loading = "Calculating directness network";
    data = await $backend!.getTownCentreRoutes();
    loading = "";
  });
</script>

<Loading {loading} />

<GeoJSON {data}>
  <LineLayer
    {...layerId("directness-network")}
    filter={["get", $showDirectness]}
    paint={{
      "line-color": constructMatchExpression(
        ["get", "los"],
        levelOfServiceColors,
        "black",
      ),
      "line-width": roadLineWidth(0),
    }}
  />
</GeoJSON>
