<script lang="ts">
  import AllPopulation from "./AllPopulation.svelte";
  import CyclingFlowCoverage from "./CyclingFlowCoverage.svelte";
  import DeprivedPopulation from "./DeprivedPopulation.svelte";
  import GpHospitals from "./GpHospitals.svelte";
  import Greenspaces from "./Greenspaces.svelte";
  import GridMeshDensity from "./GridMeshDensity.svelte";
  import MainRoadCoverage from "./MainRoadCoverage.svelte";
  import DebugNptPmtiles from "./roads/DebugNptPmtiles.svelte";
  import Roads from "./roads/Roads.svelte";
  import Schools from "./Schools.svelte";
  import Settlements from "./Settlements.svelte";
  import { cyclingFlow1, cyclingFlow2 } from "./stores";
  import TownCentres from "./TownCentres.svelte";
  import UrbanAreas from "./UrbanAreas.svelte";
</script>

<!-- Common -->

<Roads />
<DebugNptPmtiles />
<AllPopulation />
<DeprivedPopulation />

<!-- Primary -->

<CyclingFlowCoverage label="High" bind:show={$cyclingFlow1} />
<MainRoadCoverage />

<!-- Secondary -->

<CyclingFlowCoverage label="Medium" bind:show={$cyclingFlow2} />
<TownCentres />

<!-- Local acces -->

<Schools />
<GpHospitals />
<Greenspaces />

<!-- Long distance -->

<Settlements />
<UrbanAreas />

<!-- Network assessment -->

<GridMeshDensity />
