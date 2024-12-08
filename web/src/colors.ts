import { constructMatchExpression } from "svelte-utils/map";
import { infraTypes } from "./stores";

export let tierColors = {
  Primary: "#c00000",
  Secondary: "#e97132",
  LocalAccess: "#ffc000",
  LongDistance: "#4ea72e",
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

export let colorByTier = constructMatchExpression(
  ["get", "tier"],
  tierColors,
  "cyan",
);

export let colorByLoS = constructMatchExpression(
  ["get", "los"],
  levelOfServiceColors,
  "black",
);
