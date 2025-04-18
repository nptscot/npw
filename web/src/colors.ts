import { infraTypes } from "./types";

export let infraTypeColors = Object.fromEntries(
  infraTypes.map((x) => [x[0], x[2]]),
);

export let infraTypeLabels = Object.fromEntries(
  infraTypes.map((x) => [x[0], x[1]]),
);

export let infraTypeColorLegend = Object.fromEntries(
  infraTypes.map((x) => [x[1], x[2]]),
);

// TODO Making types less precise here for convenience elsewhere.
// Alternatively, improve ODStats types.

export let tierColors: { [name: string]: string } = {
  Primary: "#970f52",
  Secondary: "#ff978c",
  LocalAccess: "#feae01",
  LongDistance: "#D2691E",
};

export let stageColors = {
  ...tierColors,
  assessment: "#1b1464",
};

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

export let levelOfServiceLabels: { [name: string]: string } = {
  High: "High",
  Medium: "Medium",
  Low: "Low",
  ShouldNotBeUsed: "Should not be used",
};

export let levelOfServiceLegend: { [name: string]: string } = {
  High: levelOfServiceColors.High,
  Medium: levelOfServiceColors.Medium,
  Low: levelOfServiceColors.Low,
  "Should not be used": levelOfServiceColors.ShouldNotBeUsed,
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
  "Not enough space": "#dd7777",
  "Absolute minimum": "#e0b97d",
  "Desirable minimum": "#75a375",
};

export let deprived = {
  // Color ramp from https://simd.scot. Only show the two most deprived deciles.
  colorScale: ["#b30000", "#e34a33"],
  limits: [0, 10, 20],
};

export let population = {
  // Color ramp from https://www.ons.gov.uk/census/maps/choropleth
  colorScale: ["#080C54", "#186290", "#1F9EB7", "#80C6A3", "#CDE594"],

  // For density_quintile
  limits: [0, 1, 2, 3, 4, 5],
};

export let meshDensity = {
  colorScale: ["#d7191c", "#87d668", "#3a9120", "#054d05"],
  // Route length
  limits: [0, 1_600, 3_200, 6_400],
  // Mesh density units
  legendLimits: [">800m", "≤800m", "≤400m", "≤200m"],
};

export let componentColors = [
  "#1b9e77",
  "#d95f02",
  "#7570b3",
  "#e7298a",
  "#66a61e",
];
