<script lang="ts">
  import AllPopulation from "./AllPopulation.svelte";
  import CyclingDemandCoverage from "./CyclingDemandCoverage.svelte";
  import DeprivedPopulation from "./DeprivedPopulation.svelte";
  import GridMeshDensity from "./GridMeshDensity.svelte";
  import MainRoadCoverage from "./MainRoadCoverage.svelte";
  import MajorJunctions from "./MajorJunctions.svelte";
  import DebugNptPmtiles from "./roads/DebugNptPmtiles.svelte";
  import NetworkDisconnections from "./roads/NetworkDisconnections.svelte";
  import Roads from "./roads/Roads.svelte";
  import Settlements from "./Settlements.svelte";
  import { cyclingDemandHigh, cyclingDemandMedium } from "./stores";
  import TownCentres from "./TownCentres.svelte";
  import UncoveredPopulation from "./UncoveredPopulation.svelte";
</script>

<!-- Common -->

<Roads />
<DebugNptPmtiles />
<AllPopulation />
<DeprivedPopulation />

<!-- Primary -->

<CyclingDemandCoverage label="High" bind:show={$cyclingDemandHigh} />
<MainRoadCoverage />
<MajorJunctions />

<!-- Secondary -->

<CyclingDemandCoverage label="Medium" bind:show={$cyclingDemandMedium} />
<TownCentres />

<!-- Long distance -->

<Settlements />

<!-- Network assessment -->

<GridMeshDensity />
<NetworkDisconnections />
<UncoveredPopulation />
