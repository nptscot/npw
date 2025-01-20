<script lang="ts">
  import { QualitativeLegend } from "../../common";
  import RoadLayerControls from "../RoadLayerControls.svelte";

  // TODO Show some things if $severances

  let colors = {
    network: "green",
    reachable: "purple",
    severance: "red",
  };
</script>

<RoadLayerControls name="Reachable network" style="reachability">
  <QualitativeLegend {colors} />
</RoadLayerControls>
