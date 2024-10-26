import { writable, type Writable } from "svelte/store";

export let allControls: Writable<HTMLDivElement[]> = writable([]);
