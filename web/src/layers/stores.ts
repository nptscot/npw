import { writable, type Writable } from "svelte/store";

export let allControls: Writable<HTMLDivElement[]> = writable([]);

// Only layers that need to be opened remotely need to be here
export let schools = writable(false);
export let gpHospitals = writable(false);
export let townCentres = writable(false);
export let imdZones = writable(false);
