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

export let tierColors = {
  Primary: "#970f52",
  Secondary: "#ff978c",
  LocalAccess: "#feae01",
  LongDistance: "#badc58",
};

export let stageColors = {
  ...tierColors,
  assessment: "#1b1464",
};

export let tierLabels = {
  Primary: "Primary",
  Secondary: "Secondary",
  LocalAccess: "Local access",
  LongDistance: "Long distance",
};

// Only Primary and Secondary
export let cnTierColors = {
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

export let trafficColors = {
  UpTo1000: "#86dfdc",
  UpTo2000: "#27918d",
  UpTo4000: "#ffaa33",
  Over4000: "#440154",
};
export let trafficLabels = {
  UpTo1000: "0 - 999",
  UpTo2000: "1000 - 1999",
  UpTo4000: "2000 - 3999",
  Over4000: "4000+",
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
// TODO Make it an enum properly?
export let speedColors: { [name: number]: string } = {
  10: "#8a9a5b",
  20: "#8a9a5b",
  30: "#ffc300",
  40: "#cc5500",
  50: "#c70039",
  60: "#900c3f",
  70: "#581845",
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

export let nptStreetSpaceColors = {
  "Not enough space": "#dd7777",
  "Absolute minimum": "#e0b97d",
  "Desirable minimum": "#75a375",
};

export let deprived = {
  // Color ramp from https://simd.scot. Downscaling to 5 buckets is odd.
  colorScale: ["#980528", "#E06742", "#E9CF88", "#9EC8D8", "#426EA8"],
  limits: [0, 20, 40, 60, 80, 100],
};

export let population = {
  // Color ramp from https://www.ons.gov.uk/census/maps/choropleth
  colorScale: ["#080C54", "#186290", "#1F9EB7", "#80C6A3", "#CDE594"],

  // For density_quintile
  limits: [0, 2, 4, 6, 8, 10],
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
