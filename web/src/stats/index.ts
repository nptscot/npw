import { prettyPrintDistance } from "../common";
import type { Stats } from "../types";

export type Rating = "very poor" | "poor" | "medium" | "good" | "very good";

let ratingToPercent = {
  "very poor": 0,
  poor: 25,
  medium: 50,
  good: 75,
  "very good": 100,
};

function percent3(x: number, total: number): number {
  if (total == 0) {
    return 0;
  }

  return Math.round((x / total) * 100);
}

function stepLessThanOrEqual(pct: number, steps: number[]): Rating {
  if (pct <= steps[0]) {
    return "very poor";
  }
  if (pct <= steps[1]) {
    return "poor";
  }
  if (pct <= steps[2]) {
    return "medium";
  }
  if (pct <= steps[3]) {
    return "good";
  }
  return "very good";
}

function stepGreaterThan(pct: number, steps: number[]): Rating {
  if (pct > steps[0]) {
    return "very poor";
  }
  if (pct > steps[1]) {
    return "poor";
  }
  if (pct > steps[2]) {
    return "medium";
  }
  if (pct > steps[3]) {
    return "good";
  }
  return "very good";
}

export function safetyArterial(s: Stats): [string, Rating, number] {
  let pct = percent3(
    s.total_high_los_arterial_roads_length,
    s.total_arterial_road_length,
  );
  return [`${pct}%`, stepLessThanOrEqual(pct, [20, 40, 60, 80]), pct];
}

export function safetyPrimarySecondary(s: Stats): [string, Rating, number] {
  let pct = percent3(
    s.high_los_primary_secondary_length,
    s.total_primary_secondary_length,
  );
  return [
    `${pct}% of ${prettyPrintDistance(s.total_primary_secondary_length)}`,
    stepLessThanOrEqual(pct, [20, 40, 60, 80]),
    pct,
  ];
}

export function safetyCombined(s: Stats): [string, Rating, number] {
  let pct1 = percent3(
    s.total_high_los_arterial_roads_length,
    s.total_arterial_road_length,
  );
  let pct2 = percent3(
    s.high_los_primary_secondary_length,
    s.total_primary_secondary_length,
  );
  let pct = Math.round(0.9 * pct1 + 0.1 * pct2);
  return [`${pct}%`, stepLessThanOrEqual(pct, [20, 40, 60, 80]), pct];
}

export function coherenceDensity(s: Stats): [string, Rating, number] {
  if (!s.density_network_in_settlements) {
    return ["no routes", "very poor", 0];
  }
  let rating = stepGreaterThan(
    s.density_network_in_settlements,
    [1000, 500, 400, 250],
  );
  return [
    `${Math.round(s.density_network_in_settlements)}m`,
    rating,
    ratingToPercent[rating],
  ];
}

export function coherenceIntegrity(s: Stats): [string, Rating, number] {
  let rating: Rating = "very poor";
  if (s.num_connected_components <= s.num_settlements) {
    rating = "medium";
  } else if (s.num_connected_components == 1) {
    rating = "very good";
  }
  return [
    s.num_connected_components.toString(),
    rating,
    ratingToPercent[rating],
  ];
}

export function coherenceCombined(s: Stats): [string, Rating, number] {
  let pct1 = ratingToPercent[coherenceDensity(s)[1]];
  let pct2 = percent3(
    s.total_high_los_arterial_roads_length,
    s.total_arterial_road_length,
  );
  let pct3 = ratingToPercent[coherenceIntegrity(s)[1]];
  let pct = Math.round(0.4 * pct1 + 0.3 * pct2 + 0.3 * pct3);
  return [`${pct}%`, stepLessThanOrEqual(pct, [20, 40, 60, 80]), pct];
}

export function comfort(s: Stats): [string, Rating, number] {
  let pct = percent3(s.total_low_gradient_length, s.total_network_length);
  return [
    `${pct}% of ${prettyPrintDistance(s.total_network_length)}`,
    stepLessThanOrEqual(pct, [10, 20, 40, 60]),
    pct,
  ];
}

export function attractiveness(s: Stats): [string, Rating, number] {
  let pct = percent3(s.total_attractive_length, s.total_network_length);
  // First threshold will almost never happen; this is a deliberate choice
  return [
    `${pct}% of ${prettyPrintDistance(s.total_network_length)}`,
    stepLessThanOrEqual(pct, [0, 25, 50, 75]),
    pct,
  ];
}

export function directness(s: {
  average_weighted_directness: number;
}): [string, Rating, number] {
  let rating = stepGreaterThan(
    s.average_weighted_directness,
    [1.3, 1.2, 1.1, 1],
  );
  return [
    `${s.average_weighted_directness.toFixed(1)}x`,
    rating,
    ratingToPercent[rating],
  ];
}
