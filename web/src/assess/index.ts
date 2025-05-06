import { writable, type Writable } from "svelte/store";

type Subpage =
  | "overview"
  | "report"
  | "disconnected"
  | "mesh-density"
  | "streetspace"
  | "calculated-routes"
  | "directness-network";
export let subpage: Writable<Subpage> = writable("overview");

export let showDirectness: Writable<"direct" | "quiet"> = writable("quiet");
