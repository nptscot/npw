import { writable, type Writable } from "svelte/store";
import type { PoiKind } from "../types";

export interface POI {
  kind: PoiKind;
  idx: number;
  description: string;
  reachable: boolean;
  pt: [number, number];
}

export let currentPOI: Writable<POI | null> = writable(null);
