import { constructMatchExpression } from "svelte-utils/map";
import { infraTypes } from "./stores";

export let infraTypeColors = Object.fromEntries(
  infraTypes.map((x) => [x[0], x[2]]),
);

// TODO Making types less precise here for convenience elsewhere.
// Alternatively, improve ODStats types.
export let tierColors: { [name: string]: string } = {
  Primary: "#c00000",
  Secondary: "#e97132",
  LocalAccess: "#ffc000",
  LongDistance: "#4ea72e",
};

export let levelOfServiceColors: { [name: string]: string } = {
  High: "mediumseagreen",
  Medium: "orange",
  Low: "red",
  ShouldNotBeUsed: "brown",
};

export let gradientColors = {
  "<= 3%": "#59ee19",
  "3 - 5%": "#37a009",
  "5 - 7%": "#FFC300",
  "7 - 10%": "#C70039",
  "> 10%": "#581845",
};

export let colorByInfraType = constructMatchExpression(
  ["get", "infra_type"],
  infraTypeColors,
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

export let colorByGradientGroup = constructMatchExpression(
  ["get", "gradient_group"],
  gradientColors,
  "black",
);
