import { writable } from "svelte/store";

export let existingNetwork = writable(false);
export let coreNetwork = writable(false);
export let schools = writable(false);
export let townCentres = writable(false);
export let gpHospitals = writable(false);
