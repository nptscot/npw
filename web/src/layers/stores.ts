import { writable, type Writable } from "svelte/store";
import type { ReferenceRoadStyle } from "../stores";
import type { PoiKind } from "../types";

export let allControls: Writable<Map<string, HTMLDivElement>> = writable(
  new Map(),
);

// Only layers that need to be controlled remotely need to be here
export let debugAllCyclingDemand = writable(false);
export let debugCyclingDemandMin = writable(0);
export let cyclingDemandHigh = writable(false);
export let mainRoadCoverage = writable(false);
export let showUncovered = writable(false);

export let cyclingDemandMedium = writable(false);
export let townCentres = writable(false);

export let localPOIs = writable(false);

export let settlements = writable(false);

export let gridMeshDensity = writable(false);

export let severances = writable(false);
export let debugOriginalData = writable(false);

export let enableLayersPerStage = {
  Primary: [],
  Secondary: [townCentres],
  LocalAccess: [localPOIs],
  LongDistance: [settlements],
  assessment: [gridMeshDensity],
};

export let disableLayersPerStage = {
  Primary: [cyclingDemandHigh, mainRoadCoverage],
  Secondary: [cyclingDemandMedium, townCentres, severances],
  LocalAccess: [localPOIs, severances],
  LongDistance: [settlements, severances],
  assessment: [gridMeshDensity],
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
});

// Remember the last reference style enabled. "off" means none.
export let lastReferenceStyle: Writable<ReferenceRoadStyle> = writable("off");

export let populationStyle: Writable<"off" | "population" | "deprived"> =
  writable("off");

export interface POI {
  kind: PoiKind;
  idx: number;
  description: string;
  reachable: boolean;
  pt: [number, number];
}

export let currentPOI: Writable<POI | null> = writable(null);
