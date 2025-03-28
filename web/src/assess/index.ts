import { writable, type Writable } from "svelte/store";
import { gridMeshDensity } from "../layers/stores";
import { referenceRoadStyle } from "../stores";

type Subpage =
  | "overview"
  | "report"
  | "disconnected"
  | "mesh-density"
  | "streetspace"
  | "calculated-routes";
export let subpage: Writable<Subpage> = writable("overview");

export function changePage(page: Subpage) {
  gridMeshDensity.set(false);
  referenceRoadStyle.set("off");

  subpage.set(page);

  if (page == "disconnected") {
    referenceRoadStyle.set("disconnections");
  } else if (page == "mesh-density") {
    gridMeshDensity.set(true);
  } else if (page == "streetspace") {
    referenceRoadStyle.set("deliverability");
  } else if (page == "calculated-routes") {
    referenceRoadStyle.set("calculated_rnet");
  }
}
