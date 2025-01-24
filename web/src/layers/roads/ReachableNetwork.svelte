<script lang="ts">
  import { QualitativeLegend } from "../../common";
  import RoadLayerControls from "../RoadLayerControls.svelte";

  let colors = {
    network: "green",
    reachable: "purple",
    severance: "red",
  };
</script>

<RoadLayerControls name="Reachable network" style="reachability">
  <QualitativeLegend {colors} />
</RoadLayerControls>
