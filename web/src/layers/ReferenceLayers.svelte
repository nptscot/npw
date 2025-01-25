<script lang="ts">
  import AllPopulation from "./AllPopulation.svelte";
  import AreaMeshDensity from "./AreaMeshDensity.svelte";
  import CalculatedRouteNetwork from "./CalculatedRouteNetwork.svelte";
  import CyclingFlowCoverage from "./CyclingFlowCoverage.svelte";
  import DeprivedPopulation from "./DeprivedPopulation.svelte";
  import GpHospitals from "./GpHospitals.svelte";
  import Greenspaces from "./Greenspaces.svelte";
  import GridMeshDensity from "./GridMeshDensity.svelte";
  import NetworkDisconnections from "./NetworkDisconnections.svelte";
  import PrecalculatedRouteNetwork from "./PrecalculatedRouteNetwork.svelte";
  import CoreNetwork from "./roads/CoreNetwork.svelte";
  import ExistingNetwork from "./roads/ExistingNetwork.svelte";
  import Gradient from "./roads/Gradient.svelte";
  import LevelOfService from "./roads/LevelOfService.svelte";
  import Roads from "./roads/Roads.svelte";
  import SpeedLimit from "./roads/SpeedLimit.svelte";
  import TrafficVolume from "./roads/TrafficVolume.svelte";
  import Schools from "./Schools.svelte";
  import Settlements from "./Settlements.svelte";
  import { cyclingFlow1, cyclingFlow2, cyclingFlow3 } from "./stores";
  import TownCentres from "./TownCentres.svelte";
  import UrbanAreas from "./UrbanAreas.svelte";
</script>

<Roads />
<ExistingNetwork />
<PrecalculatedRouteNetwork />
<CalculatedRouteNetwork />
<CyclingFlowCoverage quintile={1} label="High" show={$cyclingFlow1} />
<CyclingFlowCoverage quintile={2} label="Medium" show={$cyclingFlow2} />
<CyclingFlowCoverage quintile={3} label="Above-minimum" show={$cyclingFlow3} />
<CoreNetwork />
<Schools />
<TownCentres />
<Settlements />
<Greenspaces />
<GpHospitals />
<UrbanAreas />
<LevelOfService />
<TrafficVolume />
<Gradient />
<DeprivedPopulation />
<AllPopulation />
<AreaMeshDensity />
<GridMeshDensity />
<SpeedLimit />
<NetworkDisconnections />
