import { writable, type Writable } from "svelte/store";

type Subpage =
  | "overview"
  | "report"
  | "disconnected"
  | "mesh-density"
  | "streetspace"
  | "population"
  | "calculated-routes";
export let subpage: Writable<Subpage> = writable("overview");
