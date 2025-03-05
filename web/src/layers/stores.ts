import { writable, type Writable } from "svelte/store";
import type { ReferenceRoadStyle } from "../stores";
import type { PoiKind } from "../types";

export let allControls: Writable<Map<string, HTMLDivElement>> = writable(
  new Map(),
);

// Only layers that need to be controlled remotely need to be here
export let debugAllCyclingDemand = writable(false);
export let debugCyclingDemandMin = writable(0);
export let cyclingDemand1 = writable(true);
export let mainRoadCoverage = writable(false);

export let cyclingDemand2 = writable(false);
export let townCentres = writable(false);

export let localPOIs = writable(false);

export let settlements = writable(false);

export let deprivedPopulation = writable(false);
export let allPopulation = writable(false);
export let gridMeshDensity = writable(false);

export let severances = writable(false);
export let debugOriginalData = writable(false);

export let layersPerStage = {
  // TODO We want to auto-hide this one, but not auto-show it...
  Primary: [cyclingDemand1, mainRoadCoverage],
  Secondary: [cyclingDemand2, townCentres],
  LocalAccess: [localPOIs],
  LongDistance: [settlements],
  assessment: [gridMeshDensity],
};

// Remember the last reference style enabled. "off" means none.
export let lastReferenceStyle: Writable<ReferenceRoadStyle> = writable("off");

// TODO Move to a component after merging some of the POI components
export let currentPOI: Writable<{
  kind: PoiKind;
  idx: number;
} | null> = writable(null);
