import type { Stats } from "../types";

export type Rating = "very poor" | "poor" | "medium" | "good" | "very good";

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

export function safetyArterial(s: Stats): [string, Rating] {
  let pct = percent3(
    s.total_high_los_arterial_roads_length,
    s.total_arterial_road_length,
  );
  return [`${pct}%`, stepLessThanOrEqual(pct, [20, 40, 60, 80])];
}

export function safetyPrimarySecondary(s: Stats): [string, Rating] {
  let pct = percent3(
    s.high_los_primary_secondary_length,
    s.total_primary_secondary_length,
  );
  return [`${pct}%`, stepLessThanOrEqual(pct, [20, 40, 60, 80])];
}

export function coherentDensity(s: Stats): [string, Rating] {
  if (!s.density_network_in_settlements) {
    return ["no routes", "very poor"];
  }
  let rating = stepGreaterThan(
    s.density_network_in_settlements,
    [1000, 500, 400, 250],
  );
  return [`${Math.round(s.density_network_in_settlements)}m`, rating];
}

export function comfort(s: Stats): [string, Rating] {
  let pct = percent3(s.total_low_gradient_length, s.total_network_length);
  return [`${pct}%`, stepLessThanOrEqual(pct, [10, 20, 40, 60])];
}

export function attractiveness(s: Stats): [string, Rating] {
  let pct = percent3(s.total_attractive_length, s.total_network_length);
  // First threshold will almost never happen; this is a deliberate choice
  return [`${pct}%`, stepLessThanOrEqual(pct, [0, 25, 50, 75])];
}

export function directness(s: {
  average_weighted_directness: number;
}): [string, Rating] {
  // TODO Doesn't match table
  let rating = stepGreaterThan(
    s.average_weighted_directness,
    [1.5, 1.4, 1.3, 1.2],
  );
  return [`${s.average_weighted_directness.toFixed(1)}x`, rating];
}
