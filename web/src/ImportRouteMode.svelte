<script lang="ts">
  import {
    GeoJSON,
    LineLayer,
    hoverStateFilter,
    type LayerClickInfo,
  } from "svelte-maplibre";
  import { SplitComponent } from "svelte-utils/two_column_layout";
  import { notNull } from "svelte-utils";
  import { mode, backend, coherentNetwork, autosave } from "./stores";
  import { colorByInraType } from "./common";

  // TODO Remove cases that've already been fully imported

  async function importRoute(e: CustomEvent<LayerClickInfo>) {
    let id = e.detail.features[0].id as number;
    let feature = $coherentNetwork.features[id];

    try {
      let newId = $backend!.newRoute({
        feature,
        name: "",
        notes: "",
        nodes: feature.properties!.full_path,
        infra_type: "Unknown",
      });
      await autosave();
      $mode = {
        kind: "route-details",
        id: newId,
      };
    } catch (err) {
      window.alert(err);
    }
  }
</script>

<SplitComponent>
  <div slot="sidebar">
    <h2>Import a route from the coherent network</h2>

    <button on:click={() => ($mode = { kind: "main" })}>Back</button>

    <p>
      Click a route to import it. Note this isn't a real example of a coherent
      network
    </p>
  </div>

  <div slot="map">
    <GeoJSON data={$coherentNetwork} generateId>
      <LineLayer
        id="cn"
        paint={{
          "line-width": hoverStateFilter(5, 10),
          "line-color": "red",
        }}
        manageHoverState
        hoverCursor="pointer"
        on:click={importRoute}
      />
    </GeoJSON>

    {#await notNull($backend).renderRoutes() then data}
      <GeoJSON {data}>
        <LineLayer
          id="routes"
          paint={{
            "line-width": 5,
            "line-color": colorByInraType,
          }}
        />
      </GeoJSON>
    {/await}
  </div>
</SplitComponent>
