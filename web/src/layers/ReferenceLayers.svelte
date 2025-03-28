<script lang="ts">
  import AllPopulation from "./AllPopulation.svelte";
  import CyclingDemandCoverage from "./CyclingDemandCoverage.svelte";
  import DeprivedPopulation from "./DeprivedPopulation.svelte";
  import Greenspaces from "./Greenspaces.svelte";
  import GridMeshDensity from "./GridMeshDensity.svelte";
  import MainRoadCoverage from "./MainRoadCoverage.svelte";
  import MajorJunctions from "./MajorJunctions.svelte";
  import POIs from "./POIs.svelte";
  import DebugNptPmtiles from "./roads/DebugNptPmtiles.svelte";
  import Roads from "./roads/Roads.svelte";
  import Settlements from "./Settlements.svelte";
  import { cyclingDemandHigh, cyclingDemandMedium } from "./stores";
  import TownCentres from "./TownCentres.svelte";
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

<!-- Local acces -->

<POIs />
<Greenspaces />

<!-- Long distance -->

<Settlements />

<!-- Network assessment -->

<GridMeshDensity />
