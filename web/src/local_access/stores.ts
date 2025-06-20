import type { Feature, FeatureCollection, LineString } from "geojson";
import { writable, type Writable } from "svelte/store";
import type { PoiKind, SetRouteInput } from "../types";

export interface POI {
  kind: PoiKind;
  idx: number;
  description: string;
  reachable: boolean;
  pt: [number, number];
}

export let filterKind: Writable<PoiKind | "all"> = writable("all");

export let currentPOI: Writable<POI | null> = writable(null);

export let fixCurrentPOI: Writable<Feature<
  LineString,
  SetRouteInput & { length_meters: number }
> | null> = writable(null);

export let debugReachabilityCurrentPOI: Writable<
  (FeatureCollection & { length_meters: number }) | null
> = writable(null);
