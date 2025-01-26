import type {
  DataDrivenPropertyValueSpecification,
  ExpressionSpecification,
} from "maplibre-gl";

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
    "#FF00C5",
  ] as ExpressionSpecification;
}
