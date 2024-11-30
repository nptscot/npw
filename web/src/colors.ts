import { constructMatchExpression } from "svelte-utils/map";
import { infraTypes } from "./stores";

export let tierColors = {
  primaryRoutes: "#c00000",
  secondaryRoutes: "#e97132",
  localAccessRoutes: "#ffc000",
  longDistanceRoutes: "#4ea72e",
};

export let levelOfServiceColors = {
  High: "mediumseagreen",
  Medium: "orange",
  Low: "red",
  ShouldNotBeUsed: "brown",
};

export let colorByInfraType = constructMatchExpression(
  ["get", "infra_type"],
  Object.fromEntries(infraTypes.map((x) => [x[0], x[2]])),
  "red",
);

export let colorByLoS = constructMatchExpression(
  ["get", "los"],
  levelOfServiceColors,
  "black",
);
