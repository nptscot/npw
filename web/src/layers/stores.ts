import { writable, type Writable } from "svelte/store";
import type { BackgroundLayer } from "../stores";

export let debugCyclingDemandMin = writable(0);
export let styleCyclingDemand = writable(true);
export let cyclingDemandHigh = writable(false);
export let mainRoadCoverage = writable(false);
export let showUncoveredDemand = writable(false);
export let majorJunctions = writable(false);

export let cyclingDemandMedium = writable(false);
export let townCentres = writable(false);

export let settlements = writable(false);

export let gridMeshDensity = writable(false);
export let uncoveredPopulation = writable(false);
export let townCentrePoints = writable(false);

export let severances = writable(false);
export let debugOriginalData = writable(false);

export let enableLayersPerStage = {
  Primary: [cyclingDemandHigh, majorJunctions],
  Secondary: [
    cyclingDemandMedium,
    townCentres,
    uncoveredPopulation,
    majorJunctions,
  ],
  LocalAccess: [],
  LongDistance: [settlements],
  assessment: [],
};

export let disableLayersPerStage = {
  Primary: [cyclingDemandHigh, mainRoadCoverage, majorJunctions],
  Secondary: [
    cyclingDemandMedium,
    townCentres,
    uncoveredPopulation,
    severances,
    majorJunctions,
  ],
  LocalAccess: [severances],
  LongDistance: [settlements, severances],
  assessment: [gridMeshDensity, uncoveredPopulation],
};

export let showNetworkTiers = writable({
  Primary: true,
  Secondary: true,
  LocalAccess: true,
  LongDistance: true,
});

export let showNetworkInfraTypes = writable({
  Segregated: true,
  SegregatedWithSpeedVolume: true,
  OffRoad: true,
  SharedFootway: true,
  CycleLane: true,
  MixedTraffic: true,
  MixedTrafficWithSpeedVolume: true,
});

export let showNetworkDeliverability = writable({
  deliverable: true,
  not: true,
});

export let showNetworkLoS = writable({
  High: true,
  Medium: true,
  Low: true,
  ShouldNotBeUsed: true,
});

// Remember the last reference style enabled. "off" means none.
export let lastBackgroundLayer: Writable<BackgroundLayer> = writable("off");
