import type {
  DataDrivenPropertyValueSpecification,
  ExpressionSpecification,
  LngLatBoundsLike,
} from "maplibre-gl";

export { default as BackLink } from "./BackLink.svelte";
export { default as Checkbox } from "./Checkbox.svelte";
export { default as DrawRectangle } from "./DrawRectangle.svelte";
export { default as HelpButton } from "./HelpButton.svelte";
export { default as LegendWithToggles } from "./LegendWithToggles.svelte";
export { default as LoadingSpinner } from "./LoadingSpinner.svelte";
export { default as Modal } from "./Modal.svelte";
export { default as Radio } from "./Radio.svelte";
export { layerId } from "./zorder";

// Zoom-dependant line width, adapted from from the Minor road layer (secondary
// road class) from https://api.maptiler.com/maps/streets-v2/style.json.
export function roadLineWidth(extraWidth: number): ExpressionSpecification {
  return [
    "interpolate",
    ["linear"],
    ["zoom"],
    5,
    0.5 + extraWidth,
    10,
    1 + extraWidth,
    12,
    1.5 + extraWidth,
    14,
    4 + extraWidth,
    16,
    7 + extraWidth,
    20,
    24 + extraWidth,
  ];
}

export function prettyPrintDistance(meters: number): string {
  if (meters < 1000.0) {
    return Math.round(meters) + "m";
  }
  return (meters / 1000.0).toFixed(1) + "km";
}

export function percent(x: number, total: number): string {
  if (total == 0) {
    return "0%";
  }

  let p = Math.round((x / total) * 100);
  return `${p}%`;
}

export function sum(list: number[]): number {
  return list.reduce((total, x) => total + x, 0);
}

// Implements the formula y = (3 / (1 + exp(-3 * (x / 1000 - 1.6))) + 0.3)
export function lineWidthForDemand(
  input: DataDrivenPropertyValueSpecification<number>,
): ExpressionSpecification {
  return [
    "let",
    "base",
    [
      "+",
      0.3,
      ["/", 3, ["+", 1, ["^", 2.718, ["-", 2.94, ["*", input, 0.0021]]]]],
    ],
    [
      "interpolate",
      ["linear"],
      ["zoom"],
      12,
      ["*", 2.1, ["var", "base"]],
      14,
      ["*", 5.25, ["var", "base"]],
      15,
      ["*", 7.5, ["var", "base"]],
      16,
      ["*", 18, ["var", "base"]],
      18,
      ["*", 52.5, ["var", "base"]],
    ],
  ] as ExpressionSpecification;
}

export function lineColorForDemand(
  input: DataDrivenPropertyValueSpecification<number>,
): ExpressionSpecification {
  return [
    "step",
    input,
    "rgba(0,0,0,0)",
    1,
    "#9C9C9C",
    50,
    "#FFFF73",
    100,
    "#AFFF00",
    250,
    "#00FFFF",
    500,
    "#30B0FF",
    1000,
    "#2E5FFF",
    2000,
    "#0000FF",
    3000,
    "#ba00ff",
  ] as ExpressionSpecification;
}

export function stripPrefix(value: string, prefix: string): string {
  return value.startsWith(prefix) ? value.slice(prefix.length) : value;
}

export let countryBounds = {
  scotland: [-8.943, 54.631, -0.901, 59.489] as LngLatBoundsLike,
  england: [-5.96, 49.89, 2.31, 55.94] as LngLatBoundsLike,
};
