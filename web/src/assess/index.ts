import { writable, type Writable } from "svelte/store";
import { gridMeshDensity, uncoveredPopulation } from "../layers/stores";
import { referenceRoadStyle } from "../stores";

type Subpage =
  | "overview"
  | "report"
  | "disconnected"
  | "mesh-density"
  | "streetspace"
  | "population"
  | "calculated-routes";
export let subpage: Writable<Subpage> = writable("overview");

export function changePage(page: Subpage) {
  gridMeshDensity.set(false);
  uncoveredPopulation.set(false);
  referenceRoadStyle.set("off");

  subpage.set(page);

  if (page == "disconnected") {
    referenceRoadStyle.set("disconnections");
  } else if (page == "mesh-density") {
    gridMeshDensity.set(true);
  } else if (page == "population") {
    uncoveredPopulation.set(true);
  } else if (page == "streetspace") {
    referenceRoadStyle.set("deliverability");
  } else if (page == "calculated-routes") {
    referenceRoadStyle.set("calculated_rnet");
  }
}
