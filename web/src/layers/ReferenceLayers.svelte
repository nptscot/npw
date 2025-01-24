<script lang="ts">
  import AllPopulation from "./AllPopulation.svelte";
  import AreaMeshDensity from "./AreaMeshDensity.svelte";
  import CalculatedRouteNetwork from "./CalculatedRouteNetwork.svelte";
  import CoreNetwork from "./CoreNetwork.svelte";
  import CyclingFlowCoverage from "./CyclingFlowCoverage.svelte";
  import DeprivedPopulation from "./DeprivedPopulation.svelte";
  import ExistingNetwork from "./ExistingNetwork.svelte";
  import GpHospitals from "./GpHospitals.svelte";
  import Gradient from "./Gradient.svelte";
  import Greenspaces from "./Greenspaces.svelte";
  import GridMeshDensity from "./GridMeshDensity.svelte";
  import LevelOfService from "./LevelOfService.svelte";
  import NetworkDisconnections from "./NetworkDisconnections.svelte";
  import PrecalculatedRouteNetwork from "./PrecalculatedRouteNetwork.svelte";
  import ReachableNetwork from "./ReachableNetwork.svelte";
  import Schools from "./Schools.svelte";
  import Settlements from "./Settlements.svelte";
  import SpeedLimit from "./SpeedLimit.svelte";
  import { cyclingFlow1, cyclingFlow2, cyclingFlow3 } from "./stores";
  import TownCentres from "./TownCentres.svelte";
  import TrafficVolume from "./TrafficVolume.svelte";
  import UrbanAreas from "./UrbanAreas.svelte";
</script>

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
<ReachableNetwork />
<DeprivedPopulation />
<AllPopulation />
<AreaMeshDensity />
<GridMeshDensity />
<SpeedLimit />
<NetworkDisconnections />
