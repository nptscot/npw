<script lang="ts">
  import AllPopulation from "./AllPopulation.svelte";
  import AreaMeshDensity from "./AreaMeshDensity.svelte";
  import CyclingFlowCoverage from "./CyclingFlowCoverage.svelte";
  import DeprivedPopulation from "./DeprivedPopulation.svelte";
  import GpHospitals from "./GpHospitals.svelte";
  import Greenspaces from "./Greenspaces.svelte";
  import GridMeshDensity from "./GridMeshDensity.svelte";
  import DebugNptPmtiles from "./roads/DebugNptPmtiles.svelte";
  import Roads from "./roads/Roads.svelte";
  import Schools from "./Schools.svelte";
  import Settlements from "./Settlements.svelte";
  import { cyclingFlow1, cyclingFlow2 } from "./stores";
  import TownCentres from "./TownCentres.svelte";
  import UrbanAreas from "./UrbanAreas.svelte";
</script>

<Roads />
<DebugNptPmtiles />
<CyclingFlowCoverage quintile={1} label="High" bind:show={$cyclingFlow1} />
<CyclingFlowCoverage quintile={2} label="Medium" bind:show={$cyclingFlow2} />
<Schools />
<TownCentres />
<Settlements />
<Greenspaces />
<GpHospitals />
<UrbanAreas />
<DeprivedPopulation />
<AllPopulation />
<AreaMeshDensity />
<GridMeshDensity />
