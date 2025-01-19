import { writable, type Writable } from "svelte/store";

export let allControls: Writable<Map<string, HTMLDivElement>> = writable(
  new Map(),
);

// Only layers that need to be controlled remotely need to be here
export let schools = writable(false);
export let gpHospitals = writable(false);
export let townCentres = writable(false);
export let settlements = writable(false);
export let greenspaces = writable(false);
export let deprivedPopulation = writable(false);
export let allPopulation = writable(false);
export let cyclingFlow1 = writable(false);
export let cyclingFlow2 = writable(false);
export let gridMeshDensity = writable(false);
export let areaMeshDensity = writable(false);
export let cyclingFlow3 = writable(false);
export let severances = writable(false);
