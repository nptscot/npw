import type { Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";
import * as Comlink from "comlink";
import { type Backend } from "./worker";

export let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

export type Mode = { kind: "debug" };

export let mode: Writable<Mode> = writable({ kind: "debug" });
export let map: Writable<Map | null> = writable(null);

// TODO Does this need to be a store?
export let backend: Writable<Comlink.Remote<Backend> | null> = writable(null);
// Indicates the backend is ready and a file is loaded
export let isLoaded = writable(false);
