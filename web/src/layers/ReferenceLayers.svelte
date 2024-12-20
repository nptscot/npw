<script lang="ts">
  import ExistingNetwork from "./ExistingNetwork.svelte";
  import RouteNetwork from "./RouteNetwork.svelte";
  import CoreNetwork from "./CoreNetwork.svelte";
  import Schools from "./Schools.svelte";
  import TownCentres from "./TownCentres.svelte";
  import GpHospitals from "./GpHospitals.svelte";
  import UrbanAreas from "./UrbanAreas.svelte";
  import ReachableNetwork from "./ReachableNetwork.svelte";
  import LevelOfService from "./LevelOfService.svelte";
  import TrafficVolume from "./TrafficVolume.svelte";
  import Gradient from "./Gradient.svelte";
  import DeprivedPopulation from "./DeprivedPopulation.svelte";
  import AllPopulation from "./AllPopulation.svelte";
  import CyclingFlowCoverage from "./CyclingFlowCoverage.svelte";
  import MajorJunctions from "./MajorJunctions.svelte";
  import { cyclingFlow1, cyclingFlow2, cyclingFlow3 } from "./stores";
</script>

<ExistingNetwork />
<RouteNetwork />
<CyclingFlowCoverage quintile={1} label="High" show={$cyclingFlow1} />
<CyclingFlowCoverage quintile={2} label="Medium" show={$cyclingFlow2} />
<CyclingFlowCoverage quintile={3} label="Above-minimum" show={$cyclingFlow3} />
<CoreNetwork />
<Schools />
<TownCentres />
<GpHospitals />
<UrbanAreas />
<LevelOfService />
<TrafficVolume />
<Gradient />
<ReachableNetwork />
<DeprivedPopulation />
<AllPopulation />
<MajorJunctions />
