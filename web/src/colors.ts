import { infraTypes } from "./types";

export let infraTypeColors = Object.fromEntries(
  infraTypes.map((x) => [x[0], x[2]]),
);

export let infraTypeLabels = Object.fromEntries(
  infraTypes.map((x) => [x[0], x[1]]),
);

// TODO Making types less precise here for convenience elsewhere.
// Alternatively, improve ODStats types.

export let tierColors: { [name: string]: string } = {
  Primary: "#970f52",
  Secondary: "#ff978c",
  LocalAccess: "#feae01",
  LongDistance: "#fef157",
};
export let networkAssessmentColor = "#1b1464";

export let tierLabels: { [name: string]: string } = {
  Primary: "Primary",
  Secondary: "Secondary",
  LocalAccess: "Local access",
  LongDistance: "Long distance",
};

// Only Primary and Secondary
export let cnTierColors: { [name: string]: string } = {
  Primary: tierColors.Primary,
  Secondary: tierColors.Secondary,
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

export let traffic = {
  colorScale: ["#27918d", "#ffaa33", "#440154"],
  limits: [0, 2000, 4000, 10000],
};

export let speed = {
  colorScale: [
    "#8a9a5b",
    "#ffc300",
    "#cc5500",
    "#c70039",
    "#900c3f",
    "#581845",
  ],
  limits: [20, 30, 40, 50, 60, 70],
};

export let gradient = {
  colorScale: [
    "#59ee19",
    "#37a009",
    "#FFC300",
    "#C70039",
    "#581845",
    "#000000",
  ],
  limits: [3, 5, 7, 10, 100],
};

export let reachabilityColors = {
  network: "green",
  reachable: "purple",
  severance: "red",
};

export let streetSpaceColors = {
  nothing: "red",
  Segregated: "green",
};

export let nptStreetSpaceColors = {
  "Not enough space": "red",
  "Absolute minimum": "yellow",
  "Desirable minimum": "green",
};

export let deprived = {
  // Color ramp from https://www.ons.gov.uk/census/maps/choropleth. Lowest value is the worst (darkest).
  colorScale: ["#080C54", "#186290", "#1F9EB7", "#80C6A3", "#CDE594"],

  // The percentiles are [1, 20]. The 5 colors cover 4 each.
  limits: [0, 4, 8, 12, 16, 20],
};

export let population = {
  colorScale: deprived.colorScale,

  // For density_quintile
  limits: [0, 1, 2, 3, 4, 5],
};
