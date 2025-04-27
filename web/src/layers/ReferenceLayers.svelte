<script lang="ts">
  import GridMeshDensity from "./GridMeshDensity.svelte";
  import MajorJunctions from "./MajorJunctions.svelte";
  import Population from "./Population.svelte";
  import DebugNptPmtiles from "./roads/DebugNptPmtiles.svelte";
  import NetworkDisconnections from "./roads/NetworkDisconnections.svelte";
  import Roads from "./roads/Roads.svelte";
  import Settlements from "./Settlements.svelte";
  import TownCentrePoints from "./TownCentrePoints.svelte";
  import TownCentres from "./TownCentres.svelte";
  import UncoveredPopulation from "./UncoveredPopulation.svelte";
</script>

<!-- Common -->

<Roads />
<DebugNptPmtiles />
<Population />

<!-- Primary -->

<MajorJunctions />

<!-- Secondary -->

<TownCentres />

<!-- Long distance -->

<Settlements />

<!-- Network assessment -->

<GridMeshDensity />
<NetworkDisconnections />
<UncoveredPopulation />
<TownCentrePoints />
