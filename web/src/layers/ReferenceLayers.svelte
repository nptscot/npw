<script lang="ts">
  import AllPopulation from "./AllPopulation.svelte";
  import CyclingDemandCoverage from "./CyclingDemandCoverage.svelte";
  import DeprivedPopulation from "./DeprivedPopulation.svelte";
  import Greenspaces from "./Greenspaces.svelte";
  import GridMeshDensity from "./GridMeshDensity.svelte";
  import MainRoadCoverage from "./MainRoadCoverage.svelte";
  import POIs from "./POIs.svelte";
  import DebugNptPmtiles from "./roads/DebugNptPmtiles.svelte";
  import Roads from "./roads/Roads.svelte";
  import Settlements from "./Settlements.svelte";
  import { cyclingDemand1, cyclingDemand2 } from "./stores";
  import TownCentres from "./TownCentres.svelte";
</script>

<!-- Common -->

<Roads />
<DebugNptPmtiles />
<AllPopulation />
<DeprivedPopulation />

<!-- Primary -->

<CyclingDemandCoverage label="High" bind:show={$cyclingDemand1} />
<MainRoadCoverage />

<!-- Secondary -->

<CyclingDemandCoverage label="Medium" bind:show={$cyclingDemand2} />
<TownCentres />

<!-- Local acces -->

<POIs />
<Greenspaces />

<!-- Long distance -->

<Settlements />

<!-- Network assessment -->

<GridMeshDensity />
