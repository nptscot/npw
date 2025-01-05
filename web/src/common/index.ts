import type { ExpressionSpecification } from "maplibre-gl";

export { default as HelpButton } from "./HelpButton.svelte";
export { default as QualitativeLegend } from "./QualitativeLegend.svelte";
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
