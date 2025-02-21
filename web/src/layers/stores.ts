import { writable, type Writable } from "svelte/store";
import type { ReferenceRoadStyle } from "../stores";

export let allControls: Writable<Map<string, HTMLDivElement>> = writable(
  new Map(),
);

// Only layers that need to be controlled remotely need to be here
export let debugAllCyclingFlow = writable(false);
export let debugCyclingFlowMin = writable(0);
export let cyclingFlow1 = writable(true);
export let mainRoadCoverage = writable(false);

export let cyclingFlow2 = writable(false);
export let townCentres = writable(false);

export let localPOIs = writable(false);

export let settlements = writable(false);

export let deprivedPopulation = writable(false);
export let allPopulation = writable(false);
export let gridMeshDensity = writable(false);

export let severances = writable(false);
export let debugOriginalData = writable(false);

export let layersPerTier = {
  // TODO We want to auto-hide this one, but not auto-show it...
  Primary: [cyclingFlow1, mainRoadCoverage],
  Secondary: [cyclingFlow2, townCentres],
  LocalAccess: [localPOIs],
  LongDistance: [settlements],
};

// Remember the last reference style enabled. "off" means none.
export let lastReferenceStyle: Writable<ReferenceRoadStyle> = writable("off");
